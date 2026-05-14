use std::collections::HashMap;

fn minimum_total_top_down(triangle: Vec<Vec<i32>>) -> i32 {
    /*
    DP Top-Down (Memoization) approach for Triangle problem.
    Starts from the top and explores paths downward.
    Time: O(n²), Space: O(n²) for memoization
    */
    let mut memo: HashMap<(usize, usize), i32> = HashMap::new();

    fn dfs(
        triangle: &Vec<Vec<i32>>,
        row: usize,
        col: usize,
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        // Base case: at the bottom
        if row == triangle.len() - 1 {
            return triangle[row][col];
        }

        // Check memo
        if let Some(&result) = memo.get(&(row, col)) {
            return result;
        }

        // Current value + minimum of two possible next positions
        let result = triangle[row][col]
            + std::cmp::min(
                dfs(triangle, row + 1, col, memo),
                dfs(triangle, row + 1, col + 1, memo),
            );
        memo.insert((row, col), result);
        result
    }

    dfs(&triangle, 0, 0, &mut memo)
}

fn main() {
    let triangle = vec![
        vec![2],
        vec![3, 4],
        vec![6, 5, 7],
        vec![4, 1, 8, 3],
    ];
    println!("{}", minimum_total_top_down(triangle));  // 11
}
