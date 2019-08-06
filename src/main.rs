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
mod print_result;
mod puzzle;

use crate::graph::Graph;
use crate::options::AlgorithmValues;
use crate::options::HeuristicValues;
use options::Opt;
use puzzle::Puzzle;

fn open_file(filename: PathBuf) -> File {
    File::open(filename).expect("Could not open file")
}

fn file_to_vec(filename: PathBuf) -> Vec<String> {
    let file = open_file(filename);
    let br_file = BufReader::new(file);

    let lines: Vec<String> = br_file.lines().collect::<Result<_, _>>().unwrap();

    lines
}

pub fn get_heuristic(heuristic_value: &HeuristicValues) -> fn(&Puzzle, &Puzzle) -> usize {
    match heuristic_value {
        HeuristicValues::Hamming => heuristic::hamming_distance,
        HeuristicValues::Manhattan => heuristic::manhattan_distance,
        HeuristicValues::Linear => heuristic::linear_conflict,
        HeuristicValues::M2L => heuristic::manhattan_linear_conflict_heuristic,
    }
}

pub fn get_algorithm(
    algorithm_value: &AlgorithmValues,
) -> fn(Puzzle, fn(&Puzzle, &Puzzle) -> usize) {
    match algorithm_value {
        AlgorithmValues::Greedy => Graph::a_star_greedy,
        AlgorithmValues::Gluttony => Graph::a_star,
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    let heuristic = get_heuristic(&opt.heuristic);
    let algorithm = get_algorithm(&opt.algorithm);
    if opt.size != None {
        let puzzle = Puzzle::new(opt.size.unwrap());
        println!("{}", puzzle);
        algorithm(puzzle, heuristic);
    } else if opt.file != None {
        let filename = opt.file.unwrap();
        let puzzle = parser::parse(file_to_vec(filename));
        println!("{}", puzzle);
        algorithm(puzzle, heuristic);
    } else {
        panic!("Something went wrong with the parameters !");
    }
}
