## Introduction

An easy tool to walk the path and find out all file satisfying the demand, and then make them as a VLC playlist file(`.xspf`)

一个简单的工具，可以游走所目标地址，找出所有需要的文件，然后将它们制作成VLC的播放列表(`.xspf`)

## How to use

Edit the `config.yaml` first, please follow the annotation of the end of the yaml. bluray dir `BDMV` will auto detect.
And then run `python main.py`. It will output the `xspf` file in `./output/[set name].xspf`

首先需要编辑`config.yaml`，你需要根据设定文件来进行修改。蓝光文件夹`BDMV`将会自动探测
之后你只需要使用命令行工具运行`python main.py`即可。它会输出`xspf`文件到本文件夹下的`output`内