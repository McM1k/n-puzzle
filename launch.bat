cargo build --release
python generator.py -s 4 > generated_n-puzzle
start /B ./target/release/n-puzzle.exe generated_n-puzzle