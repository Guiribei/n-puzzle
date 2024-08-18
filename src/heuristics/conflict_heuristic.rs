use crate::heuristics::manhattan_distance::manhattan_distance;
use crate::models::node::Node;

fn linear_conflict(node: &Node, goal: &Node) -> i32 {
    let size = node.puzzle_configuration.len();
    let mut linear_conflict = 0;

    // Calculate row conflicts
    for row in 0..size {
        let mut max_goal_col = -1;
        for col in 0..size {
            let tile = node.puzzle_configuration[row][col];
            if tile != 0 {
                let goal_row = goal.puzzle_configuration.iter().position(|r| r.contains(&tile)).unwrap();
                let goal_col = goal.puzzle_configuration[goal_row].iter().position(|&t| t == tile).unwrap();

                if goal_row == row && goal_col as isize > max_goal_col as isize {
                    max_goal_col = goal_col as i32;
                } else if goal_row == row && goal_col < max_goal_col.try_into().unwrap() {
                    linear_conflict += 2; // Add penalty for linear conflict
                }
            }
        }
    }

    // Calculate column conflicts
    for col in 0..size {
        let mut max_goal_row = -1;
        for row in 0..size {
            let tile = node.puzzle_configuration[row][col];
            if tile != 0 {
                let goal_row = goal.puzzle_configuration.iter().position(|r| r.contains(&tile)).unwrap();
                let goal_col = goal.puzzle_configuration[goal_row].iter().position(|&t| t == tile).unwrap();

                if goal_col == col && goal_row as isize > max_goal_row as isize  {
                    max_goal_row = goal_row as i32;
                } else if goal_col == col && goal_row < max_goal_row.try_into().unwrap() {
                    linear_conflict += 2; // Add penalty for linear conflict
                }
            }
        }
    }

    linear_conflict
}

pub fn conflict_heuristic(node: &Node, goal: &Node) -> i32 {
    let manhattan = manhattan_distance(node, goal);
    let conflict = linear_conflict(node, goal);
    manhattan + conflict
}