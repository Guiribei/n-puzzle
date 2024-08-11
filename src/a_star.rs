use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::generators::generate_desired_node::generate_desired_node;
use crate::generators::generate_possible_nodes::generate_possible_nodes;
use crate::models::node::Node;

pub fn a_star(heuristic_function: fn(&Node, &Node) -> i32) {
    let mut open_nodes = BinaryHeap::new();
    let mut node_map: HashMap<Node, i32> = HashMap::new(); // Map of Node to f(x)
    let mut close_nodes: HashSet<Node> = HashSet::new();

    let start_node = Node::new_mocked();
    let desired_node = generate_desired_node(3);

    open_nodes.push(start_node.clone());
    node_map.insert(start_node.clone(), start_node.depth + start_node.heuristic_value);

    while let Some(mut current_node) = open_nodes.pop() {
        if current_node.was_seen {
            panic!("at the disco")
        }
        current_node.was_seen = true;
        println!("Current node:\n{}", current_node);
        
        if current_node == desired_node {
            println!("Solution found!");
            return;
        }

        // Remove the current node from the HashMap
        node_map.remove(&current_node);

        if close_nodes.contains(&current_node) {
            continue;
        }

        close_nodes.insert(current_node.clone());

        // Generate possible nodes
        let mut possible_nodes: Vec<Node> = generate_possible_nodes(&current_node);

        for possible_node in possible_nodes.iter_mut() {
            possible_node.depth = current_node.depth + 1; // g(x)
            possible_node.heuristic_value = heuristic_function(possible_node, &desired_node); // h(x)
            let f_value = possible_node.depth + possible_node.heuristic_value; // f(x)

            if *possible_node == desired_node {
                println!("Solution found!");
                return;
            }

            if close_nodes.contains(&possible_node) {
                continue;
            }

            // Check if the node is already in the open set with a higher f(x) value
            if let Some(&existing_f_value) = node_map.get(possible_node) {
                if f_value < existing_f_value {
                    open_nodes.push(possible_node.clone());
                    node_map.insert(possible_node.clone(), f_value);
                }
            } else {
                open_nodes.push(possible_node.clone());
                node_map.insert(possible_node.clone(), f_value);
            }
        }
    }
}
