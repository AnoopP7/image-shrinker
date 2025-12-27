# image_shrinker
## Overview
This project implements functions to facilitate applying JPEG compression to a directory or single image to reduce their size to a specified amount while preserving directory structure. It provides a command line interface for utilizing the functions easily. The use case in mind is for reducing the size of several large images (especially photographs) in bulk for sharing on services with a file size limit or other purposes where small files are desired.

## Dependencies
This project makes use of several crates from crates.io:
* `clap` v4.5.41
* `image` v0.25.9
* `turbojpeg` v1.3.3

## Usage
After building (`cargo build` or `cargo build --release`, which may require CMake, a C compiler, and NASM), example usage may look like:
```
.\image-shrinker.exe -s 10.0 -c -r path/to/input/directory/ path/to/output/directory/
```
The above command would compress images in the input directory and all nested directories to a maximum size of 10.0MB and copy them to the output directory regardless of whether they were already below 10.0MB.

Run the following for help:
```
.\image-shrinker.exe --help
```

## Future Work
* Implement multithreading for improved performance with large inputs.
* Create formal testing.
* Create GUI for easy usage.