## Minimal Segmentation with FastAI

This repository contains an example of training an image segmentation model on a small set of data using custom classes, through the [FastAI](https://docs.fast.ai/) library. The steps here were the ones used for the initial training process, although some of these tips or steps (especially the ones done with the Rust library for manipulating the imagery) can probably be done natively in Python. This example uses a small set of images to look for bluebirds in the imagery. 

Images were annotated with the [PixelAnnotationTool](https://github.com/abreheret/PixelAnnotationTool/), which then had the classes in the `$filename_watershed_mask.png` renamed using the `data/image_manipulation` script to be continuous over the range of [0;n] classes to satisfy the FastAI library requirements. 

![Orig](./data/bluebirds/eval_imgs/eval_img2.png) ![Segmented](./data/bluebirds/eval_imgs/result.png)

**"Eastern Bluebird" by Kelly Colgan Azar is licensed with CC BY-ND 2.0. To view a copy of this license, visit https://creativecommons.org/licenses/by-nd/2.0/**


---
This was tested on Pop!_OS 20.04 LTS, with an NVIDIA GeForce GTX960M GPU and Intel i7-6700HQ processor. 