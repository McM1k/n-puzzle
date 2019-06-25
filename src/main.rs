use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

mod graph;
mod heuristic;
mod node;
mod options;
mod parser;
mod puzzle;

use options::Opt;
use puzzle::Puzzle;

fn open_file(filename: PathBuf) -> File {
    let file = File::open(filename).expect("Could not open file");

    file
}

fn file_to_vec(filename: PathBuf) -> Vec<String> {
    let file = open_file(filename);
    let br_file = BufReader::new(file);

    let lines: Vec<String> = br_file.lines().collect::<Result<_, _>>().unwrap();

    lines
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    if opt.size != None {
        let puzzle = Puzzle::new(opt.size.unwrap());
        println!("{}", puzzle);
    } else if opt.file != None {
        let filename = opt.file.unwrap();
        let puzzle = parser::parse(file_to_vec(filename));
        println!("{}", puzzle);
    } else {
        panic!("Something went wrong with the parameters !");
    }
}
