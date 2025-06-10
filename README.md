# Floorplanning with Simulated Annealing

This project is part of the course [Algorithms for VLSI](https://www.fib.upc.edu/en/studies/masters/master-innovation-and-research-informatics/curriculum/syllabus/AVLSI-MIRI) taught at the UPC FIB in Barcelona.
In Floorplanning the task is to place a set of rectangular modules connected by wires in the plane minimizing area $A$ (minimum bounding box) and the wirelength $W$ (sum of interconnected lengths).
The objective function is given by
$$
\alpha A + (1 - \alpha) W,
$$
where $\alpha$ is a hyperparameter controlling the importance of area and wirelength.

Here we consider rotatable modules with fixed width and height.  
A common approach is to apply [Simulated Annealing](https://en.wikipedia.org/wiki/Simulated_annealing) to a floorplan representation.
The repository contains Rust implementations of the [Normalized Polish Expression](https://janders.eecg.utoronto.ca/1387/readings/wong_fp.pdf) and the [Sequence Pair](https://ieeexplore.ieee.org/document/480159) floorplan representations.


### Installing Rust

```bash
rustup install stable
```

### Compiling the Project

build and run the code with default options
```bash
cargo run --release
```

example with arguments
```bash
cargo run --release -- --input "benchmark/n300.floor" -i 10000000 -f "sequence_pair" -a 0.8 -c -r -s -o "floorplan_sequence_pair.svg"
```

to see all command line options run
```bash
cargo run --release -- -h
```

### Example Floorplan
Here is a floorplan of the 300 module instance from [GSRC-benchmark](http://vlsicad.eecs.umich.edu/BK/GSRCbench/
) optimized for area (6.44% dead area).
![floorplan](/eval/sp_floorplan_1_10_7.svg)


### Some Results
The plots show mean, min and max dead area/wirelength of 5 repetitions.
We run Simulated Annealing with $10^7$ iterations.

![plot1](/eval/alphas_deadarea.png)

![plot2](/eval/alphas_wire.png)