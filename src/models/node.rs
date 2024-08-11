use core::fmt;
use std::cmp;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Node {
    pub puzzle_configuration: Vec<Vec<i32>>,
    pub heuristic_value: i32,
    pub depth: i32,
    pub was_seen: bool,
    pub _parent: Option<Box<Node>>,
}

impl Node {
    pub fn new_mocked() -> Node {
        Node {
            // puzzle_configuration: vec![vec![1, 2, 9, 12], vec![14, 15, 11, 8], vec![3, 6, 10, 13], vec![5, 7, 0, 4]],
            puzzle_configuration: vec![vec![4, 6, 3, 15], vec![12, 14, 8, 1], vec![5, 10, 11, 7], vec![13, 9, 0, 2]],
            heuristic_value: 0,
            depth: 0,
            was_seen: false,
            _parent: None,
        }
    }

    pub fn new(puzzle_configuration: Vec<Vec<i32>>) -> Node {
        Node {
            puzzle_configuration,
            heuristic_value: 0,
            depth: 0,
            was_seen: false,
            _parent: None,
        }
    }
}

impl cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.puzzle_configuration == other.puzzle_configuration
    }
}

impl cmp::Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We reverse the order here because BinaryHeap is a max-heap by default,
        // but we need a min-heap for A*.
        let self_f = self.depth + self.heuristic_value;
        let other_f = other.depth + other.heuristic_value;
        other_f.cmp(&self_f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.puzzle_configuration.iter() {
            for value in row.iter() {
                write!(f, "{} ", value)?;
            }
            write!(f, "\n")?;
        }
		println!("Depth: {}", self.depth);
        println!("Heuristic value: {}", self.heuristic_value);
        Ok(())
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the puzzle configuration which determines equality
        for row in &self.puzzle_configuration {
            for &value in row {
                value.hash(state);
            }
        }
    }
}
