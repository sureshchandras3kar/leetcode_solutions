pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() { return false; }

    let m = matrix.len();
    let n = matrix[0].len();
    let mut left = 0;
    let mut right = (m * n - 1) as i32;

    while left <= right {
        let mid = (left + right) / 2;
        let mid_idx = mid as usize;
        let mid_value = matrix[mid_idx / n][mid_idx % n];

        if mid_value == target { return true; }
        else if mid_value < target { left = mid + 1; }
        else { right = mid - 1; }
    }

    false
}
