use crate::puzzle::Puzzle;
use crate::heuristic;
extern crate getopts;

struct Inputs {
    heuristic: fn(puzzle: &Puzzle) -> usize,
    puzzle: Puzzle,
}

impl Inputs {
    pub fn new(args: Vec<String>) -> Inputs {
        let mut inputs= Inputs{ heuristic:  heuristic::hamming_distance, puzzle: Puzzle::new(3)};

        let opts = [
            getopts::optopt("g", "generate", "generate and resolve a puzzle of given size"),
            getopts::optopt("h", "heuristic", "use your chosen heuristic (default : hamming)\nPossible answers :\n - hamming\n - manhattan\n - linear"),
            getopts::opt("", "help", "display this help message"),
        ];

        let matches = match getopts::getopts(args.tail(), &opts) {
            Ok(m) => m,
            Err(f) => {
                panic!("{}", f);
            }
        };

        if matches.opt_present("help") {
            println!("n-puzzle - solve given or generated n puzzle");
            println!();
            println!("Usage:");
            println!(" ./cargo run -- [SHORT-OPTION]... [STRING]...");
            println!(" ./cargo run -- LONG-OPTION");
            println!();
            println(getopts::usage("Print solution to standard output.", &opts)
                .as_slice());
            break;
        }

        if matches.opt_present("heuristic") {
            heuristic_string = matches.free.get(0);
            if heuristic_string == "hamming" {
                inputs.heuristic = heuristic::hamming_distance;
            } else if heuristic_string == "manhattan" {
                inputs.heuristic = heuristic::manhattan_distance;
            } else if heuristic_string == "linear" {
                inputs.heuristic = heuristic::manhattan_linear_conflict_heuristic;
            } else {
                panic!("wrong heuristic !");
            }
        }

        if matches.opt_present("generate") {
            println!("n-puzzle - solve given or generated n puzzle");
            println!();
            println!("Usage:");
            println!(" ./cargo run -- [SHORT-OPTION]... [STRING]...");
            println!(" ./cargo run -- LONG-OPTION");
            println!();
            println(getopts::usage("Print solution to standard output.", &opts)
                .as_slice());
            break;
        }

        if !matches.free.is_empty() {
            //^ `matches.free` contains all the arguments that are not options.
            let string = matches.free.connect(" ");
            print(string.as_slice());
        }
        return inputs;
    }
}
