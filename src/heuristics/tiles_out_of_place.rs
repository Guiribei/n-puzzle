use crate::models::node::Node;

pub fn tiles_out_of_place(current: &Node, goal: &Node) -> i32 {
    let mut count = 0;
    let puzzle_size = current.puzzle_configuration.len();

    for i in 0..puzzle_size {
        for j in 0..puzzle_size {
            let current_tile = current.puzzle_configuration[i][j];
            let goal_tile = goal.puzzle_configuration[i][j];
            if current_tile != 0 && current_tile != goal_tile {
                count += 1;
            }
        }
    }
    count
}