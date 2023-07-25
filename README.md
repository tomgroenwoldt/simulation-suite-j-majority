# Simulation of Distributed Majority Protocols

This repository contains my bachelor thesis written in LaTeX and the used simulation software.

## Tooling
The developed tools depend on my [crate](https://crates.io/) called
`simulation`, which holds all the logic for simulating the *j*-Majority process.
The `simulation_runner` binary replaces the usage of bash scripts and helps
exporting the simulated data. The `export` binary processes this data and
produces high quality `LaTeX` plots. Everything is written in [Rust](https://rust-lang.org).
### Simulation runner
![simulation_runner](https://github.com/tomgroenwoldt/bachelor-thesis/assets/70777530/46a7b21b-182a-4b8e-a545-67f4b7a4846a)

The `simulation_runner` accepts a variety of input flags. To get started execute:
```bash
./simulation_runner --help
```
All simulated data is stored in `JSON` format inside `output/<your-folder>/simulation.json`. The respective folder
is supplied via the `--output` flag.

### Export
![export](https://github.com/tomgroenwoldt/bachelor-thesis/assets/70777530/b48428de-2a62-42c7-ac74-9a0323576524)

The `export` binary takes the produced data of the `simulation_runner` as input
via `--input output/<your-folder>/simulation.json` and produces a plot specified by the user.
To list all available plots run:
```bash
./export --help
```

### Examples
#### K-Plot
```bash
./simulation_runner --total-k 50 --k-step-size 2 --total-j --j-step-size 3 --output example-k-plot
```
```bash
./export --input output/example-k-plot/simulation.json k
```

#### N-Plot
```bash
./simulation_runner --n 1000 --n-step-size 1000 --total-n 100000 --total-j 12 --output example-n-plot
```
```bash
./export --input output/example-n-plot/simulation.json n
```

#### "Triangle"-Initial-Configuration
You can still utilize bash scripts for specific scenarios like simulating all
possible initial consensus configurations and mapping them to a color between
red and green, depending on the time (interaction count) to reach consensus:
```bash
rm -r output/triangle
n=100000
for ((k = 0; k <= n; k+=5000)); do
  for ((l = 0; l <= n - k; l+=5000)); do
    if [[ $((k + l)) -le $n ]]; then
      m=$((n - k - l))
      ./target/release/simulation_runner -n $n -j 3 -k 3 --initial-config $k,$l,$m --output triangle
    fi
  done
done
```

The `export` binary still has to support your desired plot output though:
```bash
./export --input output/example-n-plot/simulation.json triangle
```

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
