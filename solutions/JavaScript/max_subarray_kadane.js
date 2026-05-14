/**
 * Kadane's Algorithm - Find the maximum sum of any contiguous subarray.
 *
 * Time Complexity: O(n) - Single pass through the array
 * Space Complexity: O(1) - Only tracking current and max sums
 *
 * @param {number[]} nums - Array of integers
 * @returns {number} Maximum subarray sum
 */
function maxSubArray(nums) {
    let maxSum = nums[0];
    let currentSum = nums[0];

    for (let i = 1; i < nums.length; i++) {
        // Decide whether to extend the existing subarray or start fresh
        currentSum = Math.max(nums[i], currentSum + nums[i]);
        // Update overall maximum
        maxSum = Math.max(maxSum, currentSum);
    }

    return maxSum;
}

module.exports = maxSubArray;
