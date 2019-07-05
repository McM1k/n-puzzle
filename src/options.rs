extern crate clap;
extern crate structopt;

use clap::arg_enum;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "A* driven n puzzle solver.")]
pub struct Opt {
    #[structopt(
        short = "h",
        long = "heuristic",
        default_value = "hamming",
        raw(possible_values = "&HeuristicValues::variants()"),
        case_insensitive = true
    )]
    pub heuristic: HeuristicValues,

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
