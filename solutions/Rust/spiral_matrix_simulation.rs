fn spiral_matrix_simulation(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let mut result = vec![];
    let mut top = 0;
    let mut bottom = matrix.len() as i32 - 1;
    let mut left = 0;
    let mut right = matrix[0].len() as i32 - 1;

    while top <= bottom && left <= right {
        // Traverse right
        for col in left..=right {
            result.push(matrix[top as usize][col as usize]);
        }
        top += 1;

        // Traverse down
        for row in top..=bottom {
            result.push(matrix[row as usize][right as usize]);
        }
        right -= 1;

        // Traverse left (if there's a row remaining)
        if top <= bottom {
            for col in (left..=right).rev() {
                result.push(matrix[bottom as usize][col as usize]);
            }
            bottom -= 1;
        }

        // Traverse up (if there's a column remaining)
        if left <= right {
            for row in (top..=bottom).rev() {
                result.push(matrix[row as usize][left as usize]);
            }
            left += 1;
        }
    }

    result
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let result = spiral_matrix_simulation(&matrix);
    println!("{:?}", result);
}
