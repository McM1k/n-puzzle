extern crate clap;
extern crate structopt;

use clap::arg_enum;
use std::path::PathBuf;
use structopt::StructOpt;
use std::fmt;

#[derive(StructOpt, Debug)]
#[structopt(about = "A* driven n puzzle solver.")]
pub struct Opt {
    #[structopt(
        short = "h",
        long = "heuristic",
        default_value = "manhattan",
        raw(possible_values = "&HeuristicValues::variants()"),
        case_insensitive = true
    )]
    pub heuristic: HeuristicValues,

    #[structopt(
        short = "a",
        long = "algorithm",
        default_value = "astar",
        raw(possible_values = "&AlgorithmValues::variants()"),
        case_insensitive = true
    )]
    pub algorithm: AlgorithmValues,

    #[structopt(short = "g", long = "generate", conflicts_with = "FILE")]
    pub size: Option<usize>,

    #[structopt(name = "FILE", parse(from_os_str), conflicts_with = "generate")]
    pub file: Option<PathBuf>,
}

arg_enum! {
    #[derive(Debug)]
    pub enum HeuristicValues {
        Hamming,
        Manhattan,
        Linear,
        M2L,
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum AlgorithmValues {
        Greedy,
        Astar,
    }
}

impl fmt::Display for Opt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Heuristic : {}", self.heuristic)?;
        writeln!(f, "Algorithm : {}", self.algorithm)?;
        if self.size != None {
            writeln!(f, "Size : {}", self.size.unwrap())?;
        }
        if self.file != None {
            writeln!(f, "File : {:?}", self.file.clone().unwrap())?;
        }
        Ok(())
    }
}