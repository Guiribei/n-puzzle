use crate::models::node::Node;

pub fn generate_desired_node(_size: i32) -> Node {
	Node::new(vec![vec![1, 2, 3], vec![8, 0, 4], vec![7, 6, 5]])
}