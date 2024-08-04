use crate::models::node::Node;

pub fn manhattan_distance(current: &Node, goal: &Node) -> i32 {
    let mut distance = 0;
    let puzzle_size = current.puzzle_configuration.len();

    for i in 0..puzzle_size {
        for j in 0..puzzle_size {
            let tile = current.puzzle_configuration[i][j];
            if tile != 0 {
                // Don't calculate distance for the empty space
                // Find the goal position of this tile
                let (goal_x, goal_y) = find_position(&goal.puzzle_configuration, tile);
                // Calculate the Manhattan distance for this tile
                distance += (i as i32 - goal_x as i32).abs() + (j as i32 - goal_y as i32).abs();
            }
        }
    }
    distance
}

fn find_position(goal: &Vec<Vec<i32>>, tile: i32) -> (usize, usize) {
    for i in 0..goal.len() {
        for j in 0..goal[i].len() {
            if goal[i][j] == tile {
                return (i, j);
            }
        }
    }
    (0, 0)
}
