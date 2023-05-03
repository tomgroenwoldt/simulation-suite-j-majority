[![codecov](https://codecov.io/gh/tomgroenwoldt/bachelor-thesis/branch/main/graph/badge.svg?token=FE4062QVEN)](https://codecov.io/gh/tomgroenwoldt/bachelor-thesis)
# Simulation of Distributed Majority Protocols
This repository contains my bachelor thesis written in LaTeX and the used simulation software.

## LaTeX

### File watcher
I'm using my daily editor for writing the thesis. I added functionality to recompile the document
on file change.

##### Dependencies
Install `inotify-tools` and the PDF viewer `evince`. On Arch this is done via:
```
  sudo pacman -S inotify-tools evince
```

Alter the `watch.sh` script to your liking. To start the file watcher run:

```
cd thesis/
./watch.sh
```

## Simulation
The simulation executes the j-Majority protocol on `n` agents with `k` possible opinions.

It consists of a GUI application built with `egui`.

#### Development
The tool is written in Rust, therefore install the toolchain for development <https://www.rust-lang.org/tools/install>.
###### Testing
To run the tests `cd` into the `simulation-cli` directory and execute:
```
cargo test
```
