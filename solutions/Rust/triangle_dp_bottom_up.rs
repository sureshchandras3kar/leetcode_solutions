fn minimum_total_bottom_up(triangle: Vec<Vec<i32>>) -> i32 {
    /*
    DP Bottom-Up approach for Triangle problem.
    Builds the solution from the bottom row upward.
    Time: O(n²), Space: O(n)
    */
    let n = triangle.len();
    let mut dp = triangle[n - 1].clone();

    // Work backwards from the second-to-last row to the top
    for i in (0..n - 1).rev() {
        for j in 0..triangle[i].len() {
            dp[j] = triangle[i][j] + std::cmp::min(dp[j], dp[j + 1]);
        }
    }

    dp[0]
}

fn main() {
    let triangle = vec![
        vec![2],
        vec![3, 4],
        vec![6, 5, 7],
        vec![4, 1, 8, 3],
    ];
    println!("{}", minimum_total_bottom_up(triangle));  // 11
}
