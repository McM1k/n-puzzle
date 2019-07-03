use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

mod print_result;
mod graph;
mod heuristic;
mod node;
mod options;
mod parser;
mod puzzle;

use crate::options::HeuristicValues;
use options::Opt;
use puzzle::Puzzle;
use crate::graph::Graph;

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

pub fn get_heuristic(heuristic_value: &HeuristicValues) -> fn(&Puzzle) -> usize {
    match heuristic_value {
        Hamming => heuristic::hamming_distance,
        Manhattan => heuristic::manhattan_distance,
        Linear => heuristic::linear_conflict,
        _ => panic!("Something went wrong with heuristic !"),
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    let heuristic = get_heuristic(&opt.heuristic);
    if opt.size != None {
        let puzzle = Puzzle::new(opt.size.unwrap());
        println!("{}", puzzle);
        Graph::a_star(puzzle, heuristic);
    } else if opt.file != None {
        let filename = opt.file.unwrap();
        let puzzle = parser::parse(file_to_vec(filename));
        println!("{}", puzzle);
        Graph::a_star(puzzle, heuristic);
    } else {
        panic!("Something went wrong with the parameters !");
    }
}
