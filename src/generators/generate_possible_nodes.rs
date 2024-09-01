use crate::models::node::Node;

pub fn generate_possible_nodes(current_node: &Node) -> Vec<Node> {
	let puzzle_configuration = &current_node.puzzle_configuration;
	let mut possible_nodes: Vec<Node> = Vec::new();
	for i in 0..puzzle_configuration.len() {
		 for j in 0.. puzzle_configuration.len() {
			if puzzle_configuration[i][j] == 0 {
				if i > 0 {
					let mut new_puzzle_configuration = puzzle_configuration.clone();
					new_puzzle_configuration[i][j] = new_puzzle_configuration[i - 1][j];
					new_puzzle_configuration[i - 1][j] = 0;
					possible_nodes.push(Node::new_with_parent(new_puzzle_configuration, current_node.clone()));
				}
				if i < puzzle_configuration.len() - 1 {
					let mut new_puzzle_configuration = puzzle_configuration.clone();
					new_puzzle_configuration[i][j] = new_puzzle_configuration[i + 1][j];
					new_puzzle_configuration[i + 1][j] = 0;
					possible_nodes.push(Node::new_with_parent(new_puzzle_configuration, current_node.clone()));
				}
				if j > 0 {
					let mut new_puzzle_configuration = puzzle_configuration.clone();
					new_puzzle_configuration[i][j] = new_puzzle_configuration[i][j - 1];
					new_puzzle_configuration[i][j - 1] = 0;
					possible_nodes.push(Node::new_with_parent(new_puzzle_configuration, current_node.clone()));
				}
				if j < puzzle_configuration.len() - 1 {
					let mut new_puzzle_configuration = puzzle_configuration.clone();
					new_puzzle_configuration[i][j] = new_puzzle_configuration[i][j + 1];
					new_puzzle_configuration[i][j + 1] = 0;
					possible_nodes.push(Node::new_with_parent(new_puzzle_configuration, current_node.clone()));
				}
			}
		}
	}
	possible_nodes
}
