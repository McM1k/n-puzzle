cargo build --release
python generator3.py -s 3 > generated_n-puzzle
start /B ./target/release/n-puzzle.exe generated_n-puzzle -a astar -h manhattan