use core::fmt;
use std::cmp;

#[derive(Clone)]
pub struct Node {
    pub puzzle_configuration: Vec<Vec<i32>>,
    pub heuristic_value: i32,
    pub depth: i32,
    pub _parent: Option<Box<Node>>,
}

impl Node {
    pub fn new_mocked() -> Node {
        Node {
            puzzle_configuration: vec![vec![7, 1, 6], vec![3, 0, 5], vec![4, 8, 2]],
            heuristic_value: 0,
            depth: 0,
            _parent: None,
        }
    }

    pub fn new(puzzle_configuration: Vec<Vec<i32>>) -> Node {
        Node {
            puzzle_configuration,
            heuristic_value: 0,
            depth: 0,
            _parent: None,
        }
    }
}

impl cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.puzzle_configuration == other.puzzle_configuration
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
        Ok(())
    }
}
