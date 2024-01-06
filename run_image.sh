cargo build --release
echo "instance,floorplan,alpha,time[ms],total_area,dead_area,total_wire,iterations,cluster_growing,recursive_bisection" > eval/images.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.8 -r -c -s -o eval/sp_floorplan_1_10_7.svg  >> eval/images.csv
./target/release/floorplanning -f sequence_pair -i 20000000 -a 0.8 -r -c -s -o eval/sp_floorplan_2_10_7.svg  >> eval/images.csv
./target/release/floorplanning -f slicing_tree  -i 10000000 -a 0.8 -r -c -s -o eval/pe_floorplan_1_10_7.svg  >> eval/images.csv
./target/release/floorplanning -f slicing_tree  -i 20000000 -a 0.8 -r -c -s -o eval/pe_floorplan_2_10_7.svg  >> eval/images.csv
