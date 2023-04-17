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
The simulation consists of a simulation CLI tool and will also include a GUI in the near future.

### CLI tool
The CLI tool takes in the following parameters:
- `--agent-count`
- `--sample-size`
- `--opinion-count`

#### Development
The tool is written in Rust, therefore install the toolchain for development <https://www.rust-lang.org/tools/install>.
###### Testing
To run the tests `cd` into the `simulation-cli` directory and execute:
```
cargo test
```
