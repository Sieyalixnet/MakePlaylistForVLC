# ONLY FOR USE BINARY 如果只是使用

Firstly download `Find_BD_Rust.exe` on the right of the page. Edit the `config.yaml` first, please follow the annotation of the end of the yaml.
Double click `Find_BD_Rust.exe` it will run.

先在右侧下载`Find_BD_Rust.zip`，根据你的需求修改`config.yaml`。然后直接左键双击`Find_BD_Rust.exe`即可


# FOR PYTHON 对于python

## Introduction

An easy tool to walk the path and find out all file satisfying the demand, and then make them as a VLC playlist file(`.xspf`)

一个简单的工具，可以游走所目标地址，找出所有需要的文件，然后将它们制作成VLC的播放列表(`.xspf`)

```shell
cd Find_BD_Python
python main.py
```

## How to use

Edit the `config.yaml` first, please follow the annotation of the end of the yaml. bluray dir `BDMV` will auto detect.
And then run `python main.py`. It will output the `xspf` file in `./output/[set name].xspf`

首先需要编辑`config.yaml`，你需要根据设定文件来进行修改。蓝光文件夹`BDMV`将会自动探测
之后你只需要使用命令行工具运行`python main.py`即可。它会输出`xspf`文件到本文件夹下的`output`内

# FOR RUST 对于Rust

Almost same as python, but in the `config.yaml` the param `FILES/path` only support `Vector<String>` instead of `String`

几乎和`python`一致，但是`FILES/path`只支持字符串数组(一个字符串也能使用数组)

The judgement of BDMV is different. But in fact, the Blu-ray dir have many unique features, so I choose easy one in Rust

判断是否为BDMV会有些不同。

BUILD:

```shell
cd Find_BD_Rust
cargo build
```

RUN:

```shell
cd Find_BD_Rust
cargo run
```