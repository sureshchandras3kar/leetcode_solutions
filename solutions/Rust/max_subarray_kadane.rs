/// Kadane's Algorithm - Find the maximum sum of any contiguous subarray.
///
/// Time Complexity: O(n) - Single pass through the array
/// Space Complexity: O(1) - Only tracking current and max sums
///
/// # Arguments
/// * `nums` - Vector of integers
///
/// # Returns
/// Maximum subarray sum
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums[0];
    let mut current_sum = nums[0];

    for i in 1..nums.len() {
        // Decide whether to extend the existing subarray or start fresh
        current_sum = current_sum.saturating_add(nums[i]).max(nums[i]);
        // Update overall maximum
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}
