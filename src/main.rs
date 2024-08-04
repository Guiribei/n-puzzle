mod a_star;

use a_star::{a_star, Node};
use rand::Rng;

fn mock_heuristic(_current_node: &Node, _desired_node: &Node) -> i32 {
    // Generate random heuristic value
    let mut rng = rand::thread_rng();
    rng.gen_range(0..100)
}

fn main() -> std::io::Result<()> {
    // let file = File::open("puzzles/puzzle3x3.solvable")?;
    // let mut buf_reader = BufReader::new(file);
    // let mut contents = String::new();
    // let _ = buf_reader.read_to_string(&mut contents);
    // println!("{contents}");
    let _a_star = a_star(mock_heuristic);
    Ok(())
}