from itertools import product

def build_rust():
    print("cargo build --release")

ex = "./target/release/floorplanning"
instance = "benchmark/n300.floor"
output_dir = "eval/"
header = "instance,floorplan,alpha,time[ms],total_area,dead_area,total_wire,iterations,cluster_growing,recursive_bisection"
algos = ["sequence_pair", "slicing_tree"]

num_alphas = 6
alphas = [round(i / (num_alphas- 1), 3) for i in range(num_alphas)]

# different alphas

# iterations = 10**7
# repeats = 5
# output_file = output_dir + "alphas.csv"
# build_rust()
# print(f"echo \"{header}\" > {output_file}")
# for _ in range(repeats):
#     for a in alphas:
#         print(f"{ex} -f sequence_pair -i {iterations} -a {a} -r -c >> {output_file}")
#         print(f"{ex} -f slicing_tree  -i {iterations} -a {a} -r -c >> {output_file}")


# different initializations

# alphas = [0.5]
# iterations = 10**7
# repeats = 5
# output_file = output_dir + "compare_init.csv"
# build_rust()
# print(f"echo \"{header}\" > {output_file}")
# for _ in range(repeats):
#     for a, algo in product(alphas, algos):
#         print(f"{ex} -f {algo} -i {iterations} -a {a}        >> {output_file}")
#         print(f"{ex} -f {algo} -i {iterations} -a {a} -r     >> {output_file}")
#         print(f"{ex} -f {algo} -i {iterations} -a {a} -r  -c >> {output_file}")

# images

# output_file = output_dir + "images.csv"
# a = 0.8

# build_rust()
# print(f"echo \"{header}\" > {output_file}")
# print(f"{ex} -f sequence_pair -i 10000000 -a {a} -r -c -s -o sp_floorplan_1_10_7.svg  >> {output_file}")
# print(f"{ex} -f sequence_pair -i 20000000 -a {a} -r -c -s -o sp_floorplan_2_10_7.svg  >> {output_file}")
# print(f"{ex} -f slicing_tree  -i 10000000 -a {a} -r -c -s -o sp_floorplan_1_10_7.svg  >> {output_file}")
# print(f"{ex} -f slicing_tree  -i 20000000 -a {a} -r -c -s -o sp_floorplan_2_10_7.svg  >> {output_file}")