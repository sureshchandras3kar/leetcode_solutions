pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    for row in matrix {
        if row.binary_search(&target).is_ok() { return true; }
    }
    false
}
