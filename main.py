import os
from typing import List, Tuple
from urllib.parse import quote

import yaml
CONFIG = {
"NAME_FILTER": [],
"AVAILABLE_SUFFIX":[]
}

def find_paths_with_dirs(base_path)->List[Tuple[str,str]]:
    paths = []
    for root, dirs, files in os.walk(base_path):
        for file in files:
            l:str = file.lower()
            suffix = l.split(".")[-1]
            if l == "index.bdmv" and "STREAM" in dirs:
                real_path = root[:-5]
                print("Find BD: ", real_path)
                paths.append(("BD",real_path.replace("\\","/")))
                break
            elif any(name in l for name in CONFIG["NAME_FILTER"]):
                print("Ignore file: ", os.path.join(root, file))
                continue
            elif any(suffix == s for s in CONFIG["AVAILABLE_SUFFIX"]):
                print("Find File: ", os.path.join(root, file))
                paths.append(("MKV",os.path.join(root, file).replace("\\","/")))
    return paths

HEAD = """<?xml version="1.0" encoding="UTF-8"?>
<playlist xmlns="http://xspf.org/ns/0/" xmlns:vlc="http://www.videolan.org/vlc/playlist/ns/0/" version="1">
	<title>Playlist</title>
	<trackList>"""
TRACKLISTEND = """
	</trackList>
    <extension application="http://www.videolan.org/vlc/playlist/0">"""
END = """
    </extension>
</playlist>"""

def removeInvalidPath(path:str):
    return path.replace("&","").replace("\"","").replace("<","").replace(">","")

def getHead(t)->str:
    if t == "BD":
        return "bluray:///"
    elif t == "MKV":
        return "file:///"
def getTrack(path,t,index)->str:
    return f"""
        <track>
            <location>{getHead(t)}{quote(path)}</location>
            <title>{removeInvalidPath(path.split("/")[-1])}</title>
            <extension application="http://www.videolan.org/vlc/playlist/0">
                <vlc:id>{str(index)}</vlc:id>
                <vlc:option>disc-caching=300</vlc:option>
            </extension>
        </track>"""

def getExtension(index):
    return f"""
        <vlc:item tid="{index}"/>"""


if __name__ == "__main__":
    with open("config.yaml",encoding="utf-8") as f:
        data = yaml.load(f, Loader=yaml.FullLoader)
    if data["NAME_FILTER"] is not None and isinstance(data["NAME_FILTER"],list):
        CONFIG["NAME_FILTER"]= data["NAME_FILTER"]
    if data["AVAILABLE_SUFFIX"] is not None and isinstance(data["AVAILABLE_SUFFIX"],list):
        CONFIG["AVAILABLE_SUFFIX"] = data["AVAILABLE_SUFFIX"]
    for i, p in enumerate(data['FILES']):
        if isinstance(p['path'],str):
            res = find_paths_with_dirs(p['path'])
        elif isinstance(p['path'],list):
            res = []
            for path in p['path']:
                res += find_paths_with_dirs(path)
        output = HEAD
        for index,item in enumerate(res):
            (t,path) = item
            output+=getTrack(path,t,index)
        output+=TRACKLISTEND
        for index in range(len(res)):
            output+=getExtension(index)
        output+=END
        os.makedirs(f"./output/",exist_ok=True)
        with open(f"./output/{p['name']}.xspf","w",encoding="utf-8") as f:
            f.write(output)
            print("finished: ", f"./output/{p['name']}.xspf")