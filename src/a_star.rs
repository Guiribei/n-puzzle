use core::fmt;
use std::cmp;

#[derive(Clone)]
pub struct Node {
    puzzle_configuration: Vec<Vec<i32>>,
    heuristic_value: i32,
    depth: i32,
    parent: Option<Box<Node>>,
}

impl Node {
    pub fn new_mocked() -> Node {
        Node {
            puzzle_configuration: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]],
            heuristic_value: 0,
            depth: 0,
            parent: None,
        }
    }

	pub fn new(puzzle_configuration: Vec<Vec<i32>>) -> Node {
		Node {
			puzzle_configuration,
			heuristic_value: 0,
			depth: 0,
			parent: None,
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
		Ok(())
	}
}

fn generate_desired_node(_size: i32) -> Node {
	Node::new(vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]])
}


fn generate_possible_nodes(current_node: &Node) -> Vec<Node> {
	let puzzle_configuration = &current_node.puzzle_configuration;
	let mut possible_nodes: Vec<Node> = Vec::new();
	for i in 0..puzzle_configuration.len() {
		 for j in 0.. puzzle_configuration.len() {
			if puzzle_configuration[i][j] == 0 {
				if i > 0 {
					let mut new_puzzle_configuration = puzzle_configuration.clone();
					new_puzzle_configuration[i][j] = new_puzzle_configuration[i - 1][j];
					new_puzzle_configuration[i - 1][j] = 0;
					possible_nodes.push(Node::new(new_puzzle_configuration));
				}
				if i < puzzle_configuration.len() - 1 {
					let mut new_puzzle_configuration = puzzle_configuration.clone();
					new_puzzle_configuration[i][j] = new_puzzle_configuration[i + 1][j];
					new_puzzle_configuration[i + 1][j] = 0;
					possible_nodes.push(Node::new(new_puzzle_configuration));
				}
				if j > 0 {
					let mut new_puzzle_configuration = puzzle_configuration.clone();
					new_puzzle_configuration[i][j] = new_puzzle_configuration[i][j - 1];
					new_puzzle_configuration[i][j - 1] = 0;
					possible_nodes.push(Node::new(new_puzzle_configuration));
				}
				if j < puzzle_configuration.len() - 1 {
					let mut new_puzzle_configuration = puzzle_configuration.clone();
					new_puzzle_configuration[i][j] = new_puzzle_configuration[i][j + 1];
					new_puzzle_configuration[i][j + 1] = 0;
					possible_nodes.push(Node::new(new_puzzle_configuration));
				}
			}
		}
	}
	possible_nodes
}

pub fn a_star(heurisitc_function: fn(&Node, &Node) -> i32) {
    let mut open_nodes: Vec<Node> = vec![Node::new_mocked()];
    let mut close_nodes: Vec<Node> = Vec::new();

	let desired_node = generate_desired_node(3);

    while !open_nodes.is_empty() {
        let current_node = open_nodes.iter().min_by_key(|node| node.heuristic_value + node.depth);
		let current_node = match current_node {
			Some(node) => node.clone(),
			None => {
				println!("No solution found!");
				return;
			},
		};
		if current_node == desired_node {
			println!("Solution found!");
			return;
		}

		// Remove the current node from open set
		if let Some(pos) = open_nodes.iter().position(|node| *node == current_node) {
			open_nodes.remove(pos);
		}

		// Add the current (used) node to the close set
		close_nodes.push(current_node.clone());

		// Generate possible nodes
		let mut possible_nodes: Vec<Node> = generate_possible_nodes(&current_node);

		for possible_node in possible_nodes.iter_mut() {
			if *possible_node == desired_node {
				println!("Solution found!");
				return;
			}

			possible_node.depth = current_node.depth + 1; // g(x)
			possible_node.heuristic_value = heurisitc_function(possible_node, &desired_node); // h(x)

			// Skip node if it is in the close set
			if close_nodes.contains(&possible_node) {
				continue;
			}

			// If it is in the open set but with a higher cost, replace it
            if let Some(open_node) = open_nodes.iter().find(|&node| *node == *possible_node) {
                if possible_node.depth + possible_node.heuristic_value < open_node.depth + open_node.heuristic_value {
                    if let Some(pos) = open_nodes.iter().position(|node| *node == *possible_node) {
                        open_nodes.remove(pos);
                    }
                    open_nodes.push(possible_node.clone());
                }
            } else {
                // If it's not in the open set, add it
                open_nodes.push(possible_node.clone());
            }
		};
    }
}
