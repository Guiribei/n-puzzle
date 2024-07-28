use std::{fs::File, io::{BufRead, BufReader, Read}};

fn main() -> std::io::Result<()> {
    let file = File::open("puzzles/puzzle3x3.solvable")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);
    println!("{contents}");
    Ok(())
}