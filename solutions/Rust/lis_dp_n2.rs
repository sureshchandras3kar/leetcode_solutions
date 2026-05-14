/**
 * Find length of longest increasing subsequence using DP O(n²).
 *
 * Time Complexity: O(n²)
 * Space Complexity: O(n)
 */
pub fn lis_dp(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut dp = vec![1; nums.len()];

    for i in 1..nums.len() {
        for j in 0..i {
            if nums[j] < nums[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }

    *dp.iter().max().unwrap()
}

fn main() {
    println!("{}", lis_dp(vec![10, 9, 2, 5, 3, 7, 101, 18]));  // 4
}
