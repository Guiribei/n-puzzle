mod a_star;
mod models;
mod generators;
mod heuristics;

use a_star::a_star;
use heuristics::manhattan_distance::manhattan_distance;
use heuristics::tiles_out_of_place::tiles_out_of_place;
use heuristics::euclidean_distance::euclidean_distance;

fn main() -> std::io::Result<()> {
    // let file = File::open("puzzles/puzzle3x3.solvable")?;
    // let mut buf_reader = BufReader::new(file);
    // let mut contents = String::new();
    // let _ = buf_reader.read_to_string(&mut contents);
    // println!("{contents}");
    let _a_star = a_star(euclidean_distance);
    Ok(())
}