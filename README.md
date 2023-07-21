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
