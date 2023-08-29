# Simulation of Distributed Majority Protocols

This repository contains my bachelor thesis written in LaTeX and the used simulation software.

## Tooling
The developed tools depend on my [crate](https://crates.io/) called
`simulation`, which holds all the logic for simulating the *j*-Majority process.
The `simulation_runner` binary replaces the usage of bash scripts and helps
exporting the simulated data. The `export` binary processes this data and
produces high quality `LaTeX` plots. Everything is written in [Rust](https://rust-lang.org).
### Simulation runner
![simulation_runner_demo](https://github.com/tomgroenwoldt/simulation-suite-j-majority/assets/70777530/d44ffbfd-93f3-4d70-bffc-be3483ea473d)



The `simulation_runner` accepts a variety of input flags. To get started execute:
```bash
./simulation_runner --help
```
All simulated data is stored in `JSON` format inside `output/<your-folder>/simulation.json`. The respective folder
is supplied via the `--output` flag.

### Export
![export_demo](https://github.com/tomgroenwoldt/simulation-suite-j-majority/assets/70777530/e9bda3ee-3ffe-4bbe-be2f-4896c0e090b3)



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
![image](https://github.com/tomgroenwoldt/simulation-suite-j-majority/assets/70777530/700f3d72-9bac-45d2-af12-fa759ba38088)


#### N-Plot
```bash
./simulation_runner --n 1000 --n-step-size 1000 --total-n 100000 --total-j 12 --output example-n-plot
```
```bash
./export --input output/example-n-plot/simulation.json n
```
![image](https://github.com/tomgroenwoldt/simulation-suite-j-majority/assets/70777530/37898796-aab5-46f4-8f74-4d4ea844221a)


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
![example-triangle-plot-1](https://github.com/tomgroenwoldt/bachelor-thesis/assets/70777530/d27e38ae-64b1-42bb-99d7-d44ad156fcf4)


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

The `export` binary still has to support your desired plot output though:
```bash
./export --input output/example-n-plot/simulation.json triangle
```
![image](https://github.com/tomgroenwoldt/simulation-suite-j-majority/assets/70777530/f524217b-5720-49cc-8c0f-a892021357f1)

.
![image](https://github.com/tomgroenwoldt/simulation-suite-j-majority/assets/70777530/f524217b-5720-49cc-8c0f-a892021357f1)
