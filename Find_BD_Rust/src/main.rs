use std::fs;
use std::io;
use std::io::Write;
use serde_yml;
use serde;

const HEAD: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<playlist xmlns="http://xspf.org/ns/0/" xmlns:vlc="http://www.videolan.org/vlc/playlist/ns/0/" version="1">
	<title>Playlist</title>
	<trackList>
"#;

const TRACKLISTEND: &str = r#"
    </trackList>
    <extension application="http://www.videolan.org/vlc/playlist/0">
"#;

const END: &str = r#"
    </extension>
</playlist>
"#;

fn remove_invalid_path(path: &str) -> String {
    path.replace("&", "").replace("\"", "").replace("<", "").replace(">", "")
}

fn get_head(t: &str) -> &str {
    match t {
        "BD" => "bluray:///",
        "MKV" => "file:///",
        _ => unreachable!(),
    }
}

fn get_track(path: &str, t: &str, index: usize) -> String {
    format!(r#"
        <track>
            <location>{}</location>
            <title>{}</title>
            <extension application="http://www.videolan.org/vlc/playlist/0">
                <vlc:id>{}</vlc:id>
                <vlc:option>disc-caching=300</vlc:option>
            </extension>
        </track>"#, get_head(t).to_owned()+&remove_invalid_path(path), &remove_invalid_path(path.split("\\").last().unwrap()), index)
}

fn get_extension(index: usize) -> String {
    format!(r#"
        <vlc:item tid="{}"/>"#, index)
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct F{
    path:Vec<String>,
    name:String

}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Config {
    NAME_FILTER: Vec<String>,
    AVAILABLE_SUFFIX: Vec<String>,
    FILES:Vec<F>
}

impl Config {
    pub fn new() -> Config {
        Config {
            NAME_FILTER: Vec::new(),
            AVAILABLE_SUFFIX: Vec::new(),
            FILES: Vec::new(),
        }
    }
}


fn find_paths_with_dirs(CONFIG: &Config,base_path: &str) -> Vec<(String, String)> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(base_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap().to_lowercase();
            let suffix = file_name.split(".").last().unwrap().to_owned();
            if file_name == "index.bdmv" && path.parent().unwrap().file_name().unwrap().to_str().unwrap().contains("STREAM") {
                let real_path = path.parent().unwrap().parent().unwrap().to_str().unwrap().to_string();
                println!("Find BD: {}", real_path);
                paths.push(("BD".to_string(), real_path));
            } else if CONFIG.NAME_FILTER.contains(&file_name) {
                println!("Ignore file: {}", path.to_str().unwrap());
            } else if CONFIG.AVAILABLE_SUFFIX.contains(&suffix) {
                println!("Find File: {}", path.to_str().unwrap());
                paths.push(("MKV".to_string(), path.to_str().unwrap().to_string()));
            }
        } else if path.is_dir() {
            paths.extend(find_paths_with_dirs(CONFIG,path.to_str().unwrap()));
        }
    }
    paths
}



fn main() -> io::Result<()> {
    let config_file = fs::read_to_string("config.yaml")?;
    //let CONFIG = Config::new();
    //TODO match一下
    let mut CONFIG:Config = Config::new();
    match serde_yml::from_str(&config_file){
        Ok(config) => {
            CONFIG = config;
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    fs::create_dir_all("./output")?;
    for (i, p) in CONFIG.FILES.iter().enumerate() {
        let paths = &p.path;
        let mut res = Vec::new();
            for path in paths {
                res.extend(find_paths_with_dirs(&CONFIG, path.as_str()));
            }
        
        let mut output = HEAD.to_string();
        for (index, item) in res.iter().enumerate() {
            output.push_str(&get_track(&item.1, &item.0, index));
        }
        output.push_str(TRACKLISTEND);
        for index in 0..res.len() {
            output.push_str(&get_extension(index));
        }
        output.push_str(END);
        let mut file = fs::File::create(format!("./output/{}.xspf", p.name))?;
        file.write_all(output.as_bytes())?;
    }
Ok(())

}