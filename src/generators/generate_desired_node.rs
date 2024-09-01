use crate::models::node::Node;

pub fn generate_desired_node(n: usize) -> Node {
    // Create an n x n matrix filled with zeros
    let mut matrix = vec![vec![0; n]; n];

    // Initialize boundaries
    let mut top = 0;
    let mut bottom = n - 1;
    let mut left = 0;
    let mut right = n - 1;
    let mut num = 1;

    while top <= bottom && left <= right {
        // Fill the top row
        for i in left..=right {
            matrix[top][i] = num;
            num += 1;
        }
        top += 1;

        // Fill the right column
        for i in top..=bottom {
            matrix[i][right] = num;
            num += 1;
        }
        right -= 1;

        // Fill the bottom row
        if top <= bottom {
            for i in (left..=right).rev() {
                matrix[bottom][i] = num;
                num += 1;
            }
            bottom -= 1;
        }

        // Fill the left column
        if left <= right {
            for i in (top..=bottom).rev() {
                matrix[i][left] = num;
                num += 1;
            }
            left += 1;
        }
    }

	matrix[(n - 1) / 2][(n - 1) / 2] = 0;
    Node::new(matrix)
}
