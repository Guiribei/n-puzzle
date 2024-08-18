use crate::models::node::Node;

pub fn gaschnig_heuristic(node: &Node, goal: &Node) -> i32 {
    let mut state = node.puzzle_configuration.clone();
    let mut zero_pos = find_zero_position(&state);

    let mut moves = 0;

    while state != goal.puzzle_configuration {
        if state[zero_pos.0][zero_pos.1] == goal.puzzle_configuration[zero_pos.0][zero_pos.1] {
            // The blank is in the correct position, find a misplaced tile and swap with blank
            let mut swapped = false;
            for i in 0..state.len() {
                for j in 0..state[i].len() {
                    if state[i][j] != goal.puzzle_configuration[i][j] && state[i][j] != 0 {
                        swap_tiles(&mut state, zero_pos, (i, j));
                        zero_pos = (i, j);
                        swapped = true;
                        break;
                    }
                }
                if swapped {
                    break;
                }
            }
        } else {
            // Swap the blank with the correct tile for its current position
            let target_value = goal.puzzle_configuration[zero_pos.0][zero_pos.1];
            let target_pos = find_tile_position(&state, target_value);
            swap_tiles(&mut state, zero_pos, target_pos);
            zero_pos = target_pos;
        }

        moves += 1;
    }

    moves
}

fn find_zero_position(state: &Vec<Vec<i32>>) -> (usize, usize) {
    for (i, row) in state.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == 0 {
                return (i, j);
            }
        }
    }
    panic!("No zero tile found in the puzzle configuration!");
}

fn find_tile_position(state: &Vec<Vec<i32>>, tile: i32) -> (usize, usize) {
    for (i, row) in state.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == tile {
                return (i, j);
            }
        }
    }
    panic!("Tile {} not found in the puzzle configuration!", tile);
}

fn swap_tiles(state: &mut Vec<Vec<i32>>, pos1: (usize, usize), pos2: (usize, usize)) {
    let temp = state[pos1.0][pos1.1];
    state[pos1.0][pos1.1] = state[pos2.0][pos2.1];
    state[pos2.0][pos2.1] = temp;
}
