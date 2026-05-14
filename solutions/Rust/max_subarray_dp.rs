/// Dynamic Programming approach - Build up subarray sums.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1) - Optimized space version
///
/// # Arguments
/// * `nums` - Vector of integers
///
/// # Returns
/// Maximum subarray sum
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }

    let mut max_sum = nums[0];
    let mut dp = nums[0];  // max sum ending at current position

    for i in 1..n {
        dp = dp.saturating_add(nums[i]).max(nums[i]);
        max_sum = max_sum.max(dp);
    }

    max_sum
}
