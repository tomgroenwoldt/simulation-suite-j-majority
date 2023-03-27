# Bachelor thesis "`<title>`"

### File watcher
I'm using my daily editor for writing the thesis. I added functionality to recompile the document
on file change.

##### Dependencies
Install the following dependencies:
```
  sudo pacman -S inotify-tools evince
```

To start the file watcher run:

```
cd thesis/
./watch.sh

```

The `export` binary still has to support your desired plot output though:
```bash
./export --input output/example-n-plot/simulation.json triangle
```
![image](https://github.com/tomgroenwoldt/simulation-suite-j-majority/assets/70777530/f524217b-5720-49cc-8c0f-a892021357f1)

.
