
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

mod parser;

fn open_file(filename: String) -> std::io::Result<File> {
	let file = File::open(filename)?;

	Ok(file)
}

fn file_to_vec(filename: String) -> Vec<String> {
	let file = open_file(filename).unwrap();
    let file = BufReader::new(file);

    let lines: Vec<String> = file.lines().collect::<Result<_, _>>().unwrap();

    lines
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let mut lines = file_to_vec(filename.to_string());
    lines = parser::remove_comments(lines);
    println!("{:?}", lines);
}