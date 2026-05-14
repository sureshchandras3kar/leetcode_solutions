pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let n = n as usize;
    let mut dp = vec![vec![]; n + 1];
    dp[0] = vec!["".to_string()];

    for i in 1..=n {
        for j in 0..i {
            for left in &dp[j] {
                for right in &dp[i - 1 - j] {
                    dp[i].push(format!("({}){}", left, right));
                }
            }
        }
    }

    dp[n].clone()
}
