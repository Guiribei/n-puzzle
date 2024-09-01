use crate::generators::generate_desired_node::generate_desired_node;
use crate::generators::generate_possible_nodes::generate_possible_nodes;
use crate::models::node::Node;

pub fn a_star(heurisitc_function: fn(&Node, &Node) -> i32, original_node: &Node) {
    let mut open_nodes: Vec<Node> = vec![original_node.clone()];
    let mut close_nodes: Vec<Node> = Vec::new();

    let desired_node = generate_desired_node(original_node.puzzle_configuration.len());
    println!("Desired node:\n{}", desired_node);

    while !open_nodes.is_empty() {
        let current_node = open_nodes
            .iter()
            .min_by_key(|node| node.heuristic_value + node.depth);
        let current_node = match current_node {
            Some(node) => node.clone(),
            None => {
                println!("No solution found!");
                return;
            }
        };
        if current_node == desired_node {
			println!("Current node:\n{}", current_node);
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
			possible_node.depth = current_node.depth + 1; // g(x)
			possible_node.heuristic_value = heurisitc_function(possible_node, &desired_node); // h(x)
			
            if *possible_node == desired_node {
				println!("Current node:\n{}", *possible_node);
                println!("Solution found!");
                return;
            }


            // Skip node if it is in the close set
            if close_nodes.contains(&possible_node) {
                continue;
            }

            // If it is in the open set but with a higher cost, replace it
            if let Some(open_node) = open_nodes.iter().find(|&node| *node == *possible_node) {
                if possible_node.depth + possible_node.heuristic_value
                    < open_node.depth + open_node.heuristic_value
                {
                    if let Some(pos) = open_nodes.iter().position(|node| *node == *possible_node) {
                        open_nodes.remove(pos);
                    }
                    open_nodes.push(possible_node.clone());
                }
            } else {
                // If it's not in the open set, add it
                open_nodes.push(possible_node.clone());
            }
        }
        if current_node.depth % 10 == 0 {
            println!("Current depth:\n{}", current_node.depth);
        }
    }
}
