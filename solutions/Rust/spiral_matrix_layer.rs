fn spiral_matrix_layer(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let mut result = vec![];
    let m = matrix.len() as i32;
    let n = matrix[0].len() as i32;
    let layers = (m.min(n) + 1) / 2;

    for layer in 0..layers {
        let top = layer;
        let bottom = m - 1 - layer;
        let left = layer;
        let right = n - 1 - layer;

        // Traverse right
        for col in left..=right {
            result.push(matrix[top as usize][col as usize]);
        }

        // Traverse down
        for row in (top + 1)..=bottom {
            result.push(matrix[row as usize][right as usize]);
        }

        // Traverse left (if there's a row remaining)
        if top < bottom {
            for col in (left..right).rev() {
                result.push(matrix[bottom as usize][col as usize]);
            }
        }

        // Traverse up (if there's a column remaining)
        if left < right {
            for row in ((top + 1)..bottom).rev() {
                result.push(matrix[row as usize][left as usize]);
            }
        }
    }

    result
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let result = spiral_matrix_layer(&matrix);
    println!("{:?}", result);
}
