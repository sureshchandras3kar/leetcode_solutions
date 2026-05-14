/// Dynamic Programming approach: compute minimum jumps needed to reach each index.
///
/// Intuition: dp[i] = minimum number of jumps to reach index i.
/// For each index i, look back at all indices j where j + nums[j] >= i,
/// meaning from j we can reach i in one jump. Take the minimum jumps[j] + 1.
///
/// Time Complexity: O(n²) - for each index, we may check all previous indices
/// Space Complexity: O(n) - dp array
pub fn jump_game_ii_dp(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 {
        return 0;
    }

    let n = nums.len();
    let mut dp = vec![i32::MAX; n];
    dp[0] = 0;  // Start at index 0 with 0 jumps

    for i in 1..n {
        // Check all previous indices to see which can reach i
        for j in 0..i {
            if j as i32 + nums[j] >= i as i32 {  // From index j, we can reach index i
                dp[i] = std::cmp::min(dp[i], dp[j] + 1);
                break;  // Optimization: once we find a j that reaches i, we can stop
            }
        }
    }

    dp[n - 1]
}

fn main() {
    println!("{}", jump_game_ii_dp(vec![2, 3, 1, 1, 4]));        // 2
    println!("{}", jump_game_ii_dp(vec![2, 3, 0, 6, 9, 1, 2]));  // 2
    println!("{}", jump_game_ii_dp(vec![10]));                   // 0
    println!("{}", jump_game_ii_dp(vec![1, 1, 1, 0]));           // 3
}
