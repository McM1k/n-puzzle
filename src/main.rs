use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod parser;
mod node;
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
    if args.len() == 2 {
        let filename = &args[1];
        let lines = file_to_vec(filename.to_string());
        let puzzle = parser::parse(lines);
        println!("{}", puzzle);
    } else if args.len() == 3 && args[1] == "-g".to_string() {
        let puzzle = Puzzle::new(
            args[2]
                .parse::<usize>()
                .expect("Unable to parse data into u32"),
        );
        println!("{}", puzzle);
    } else {
        panic!("Something went wrong with the parameters !")
    }
}
