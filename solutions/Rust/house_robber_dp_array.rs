/**
 * Rob houses with maximum money using DP array approach.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */
pub fn house_robber_dp_array(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    if nums.len() == 1 {
        return nums[0];
    }

    let mut dp = vec![0; nums.len()];
    dp[0] = nums[0];
    dp[1] = nums[0].max(nums[1]);

    for i in 2..nums.len() {
        dp[i] = (nums[i] + dp[i - 2]).max(dp[i - 1]);
    }

    dp[nums.len() - 1]
}

fn main() {
    println!("{}", house_robber_dp_array(vec![1, 2, 3, 1]));      // 4
    println!("{}", house_robber_dp_array(vec![2, 7, 9, 3, 1]));   // 12
}
