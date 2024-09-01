mod a_star;
mod generators;
mod heuristics;
mod models;

use std::{
    env,
    fs::File,
    io::{self, BufRead, Result},
};

use a_star::a_star;
use heuristics::{
    conflict_heuristic::conflict_heuristic, gaschnig_heuristic::gaschnig_heuristic,
    manhattan_distance::manhattan_distance,
};
use models::node::Node;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!(
            "Usage: {} <file_path> [manhattan/conflict/gaschnig]",
            args[0]
        );
        return Ok(());
    }
    let heuristic_function = match args.get(2) {
        Some(heuristic) => match heuristic.as_str() {
            "manhattan" => manhattan_distance,
            "conflict" => conflict_heuristic,
            "gaschnig" => gaschnig_heuristic,
            _ => {
                eprintln!("Invalid heuristic function");
                return Ok(());
            }
        },
        None => manhattan_distance,
    };

    let file_path = &args[1];

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut data: Vec<String> = Vec::new();
    let mut puzzle_configuration: Vec<Vec<i32>> = Vec::new();

    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.splitn(2, '#').next().unwrap_or("").trim().to_string())
        .filter(|line| !line.is_empty())
        .for_each(|line| data.push(line));

    if let Some(first_number) = data.first().and_then(|line| line.parse::<i32>().ok()) {
        for line in &data[1..] {
            if (line.split_whitespace().count() as i32) != first_number {
                eprintln!("Invalid puzzle configuration");
                return Ok(());
            }
        }
        data[1..]
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|number| match number.parse::<i32>() {
                        Ok(number) => number,
                        Err(_) => -1,
                    })
                    .collect::<Vec<i32>>()
            })
            .for_each(|line| puzzle_configuration.push(line));
        for number in puzzle_configuration.iter().flatten() {
            if number < &0 {
                eprintln!("Invalid puzzle configuration");
                return Ok(());
            }
        }
    } else {
        eprintln!("Invalid puzzle configuration");
        return Ok(());
    };

    let node = Node::new(puzzle_configuration);
    a_star(heuristic_function, &node);
    Ok(())
}
