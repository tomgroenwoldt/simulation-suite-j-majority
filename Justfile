build:
	cargo build
build-release:
	cargo build --release

triangle: build-release
	#!/usr/bin/env bash
	n=100000
	for ((i = 0; i <= n; i+=1000)); do
	  for ((j = 0; j <= n- i; j+=1000)); do
	    if [[ $((i + j)) -le $n ]]; then
	      k=$((n - i - j))
	      ./target/release/simulation_runner -n $n -j 3 -k 3 --initial-config $i,$j,$k --batch-size 20 --model gossip --output triangle > /dev/null 2> /dev/null
	    fi
	  done
	done
render-triangle:
	cargo run --release --bin export -- -i output/triangle/simulation.json triangle --generate-latex

k-plot: build-release
	./target/release/simulation_runner --total-k 50 --total-j 12 --batch-size 100 --model gossip --output k-plot
	./target/release/simulation_runner --total-k 50 --total-j 12 --batch-size 100 --model population --output k-plot
render-k-plot:
	cargo run --release --bin export -- -i output/k-plot/simulation.json k --generate-latex
render-k-plot-with-error-bars:
	cargo run --release --bin export -- -i output/k-plot/simulation.json k --generate-latex --error-bars

j-plot: build-release
	./target/release/simulation_runner --total-k 12 --total-j 50 --batch-size 100 --model gossip --output j-plot
	./target/release/simulation_runner --total-k 12 --total-j 50 --batch-size 100 --model population --output j-plot
render-j-plot:
	cargo run --release --bin export -- -i output/j-plot/simulation.json j --generate-latex

n-plot: build-release
	# ./target/release/simulation_runner --n 100000 --n-step-size 100000 --total-n 1000000 --total-j 12 --batch-size 100 --model population --output n-plot
	./target/release/simulation_runner --n 100000 --n-step-size 100000 --total-n 1000000 --total-j 12 --batch-size 100 --model gossip --output n-plot
render-n-plot:
	cargo run --release --bin export -- -i output/n-plot/simulation.json n --generate-latex

entropy-j-plot: build-release
	./target/release/simulation_runner --n 100000 --total-j 12 --batch-size 50 --model gossip --output entropy-j-plot
render-entropy-j-plot:
	cargo run --release --bin export -- -i output/entropy-j-plot/simulation.json entropy-over-j --generate-latex

entropy-k-plot: build-release
	./target/release/simulation_runner --n 100000 --total-k 12 --batch-size 50 --model population --output entropy-k-plot
render-entropy-k-plot:
	cargo run --release --bin export -- -i output/entropy-k-plot/simulation.json entropy-over-k --generate-latex

entropy-n-plot: build-release
	./target/release/simulation_runner --n 100000 --n-step-size 100000 --total-n 1000000 --batch-size 50 --model gossip --output entropy-n-plot
render-entropy-n-plot:
	cargo run --release --bin export -- -i output/entropy-n-plot/simulation.json entropy-over-n --generate-latex
