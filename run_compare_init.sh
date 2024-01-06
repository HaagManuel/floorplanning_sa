cargo build --release
echo "instance,floorplan,alpha,time[ms],total_area,dead_area,total_wire,iterations,cluster_growing,recursive_bisection" > eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5        >> eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5 -r     >> eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5 -r  -c >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5        >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5 -r     >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5 -r  -c >> eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5        >> eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5 -r     >> eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5 -r  -c >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5        >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5 -r     >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5 -r  -c >> eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5        >> eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5 -r     >> eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5 -r  -c >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5        >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5 -r     >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5 -r  -c >> eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5        >> eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5 -r     >> eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5 -r  -c >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5        >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5 -r     >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5 -r  -c >> eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5        >> eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5 -r     >> eval/compare_init.csv
./target/release/floorplanning -f sequence_pair -i 10000000 -a 0.5 -r  -c >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5        >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5 -r     >> eval/compare_init.csv
./target/release/floorplanning -f slicing_tree -i 10000000 -a 0.5 -r  -c >> eval/compare_init.csv
