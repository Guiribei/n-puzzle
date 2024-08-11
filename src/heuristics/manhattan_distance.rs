use std::collections::HashMap;
use crate::models::node::Node;

pub fn manhattan_distance(current: &Node, goal: &Node) -> i32 {
    let mut distance = 0;
    let puzzle_size = current.puzzle_configuration.len();

    // Precompute goal positions
    let mut goal_positions: HashMap<i32, (usize, usize)> = HashMap::new();
    for i in 0..puzzle_size {
        for j in 0..puzzle_size {
            let tile = goal.puzzle_configuration[i][j];
            goal_positions.insert(tile, (i, j));
        }
    }

    for i in 0..puzzle_size {
        for j in 0..puzzle_size {
            let tile = current.puzzle_configuration[i][j];
            if tile != 0 {
                // Get the goal position from the precomputed map
                let (goal_x, goal_y) = goal_positions[&tile];
                // Calculate the Manhattan distance for this tile
                distance += (i as i32 - goal_x as i32).abs() + (j as i32 - goal_y as i32).abs();
            }
        }
    }
    distance
}