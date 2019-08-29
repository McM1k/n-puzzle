#!/usr/bin/env bash
cargo build --release
python generator.py -s 3 > generated_n-puzzle
./target/release/n-puzzle generated_n-puzzle