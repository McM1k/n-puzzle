use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod parser;
mod puzzle;

use puzzle::Puzzle;

fn open_file(filename: String) -> File {
    let file = File::open(filename).expect("Could not open file");

    file
}

fn file_to_vec(filename: String) -> Vec<String> {
    let file = open_file(filename);
    let br_file = BufReader::new(file);

    let lines: Vec<String> = br_file.lines().collect::<Result<_, _>>().unwrap();

    lines
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let lines = file_to_vec(filename.to_string());
    let mut puzzle = parser::parse(lines);
    println!("{}", puzzle);
    puzzle = Puzzle::new(3);
    println!("{}", puzzle);
}
