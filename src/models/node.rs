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
            puzzle_configuration: vec![vec![14, 0, 1, 6], vec![3, 12, 11, 4], vec![2, 13, 8, 15], vec![5, 7, 9, 10]],
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
        let self_f = self.depth + self.heuristic_value;
        let other_f = other.depth + other.heuristic_value;

        // Compare f(x)
        if self_f == other_f {
            // Tie-breaking by h(x)
            if self.heuristic_value == other.heuristic_value {
                // Further tie-breaking by depth (g(x))
                self.depth.cmp(&other.depth)
            } else {
                other.heuristic_value.cmp(&self.heuristic_value)
            }
        } else {
            other_f.cmp(&self_f)
        }
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
        println!("f(x): {}", self.depth + self.heuristic_value);
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
