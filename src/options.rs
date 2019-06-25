extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "A* driven n puzzle solver.")]
pub struct Opt {
    #[structopt(short = "h", long = "heuristic", default_value = "hamming")]
    pub heuristic: String,

    #[structopt(short = "g", long = "generate", conflicts_with = "FILE")]
    pub size: Option<usize>,

    #[structopt(name = "FILE", parse(from_os_str), conflicts_with = "generate")]
    pub file: Option<PathBuf>,
}
