use std::io::BufRead;
use std::fs::File;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = File::open(filename)?;
    let file = std::io::BufReader::new(file);

    let lines: Vec<String> = file.lines().collect::<Result<_, _>>().unwrap();
    println!("{:?}", lines);

    Ok(())
}