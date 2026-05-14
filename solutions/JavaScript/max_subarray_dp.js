/**
 * Dynamic Programming approach - Build up subarray sums.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1) - Optimized space version
 *
 * @param {number[]} nums - Array of integers
 * @returns {number} Maximum subarray sum
 */
function maxSubArray(nums) {
    const n = nums.length;
    if (n === 1) return nums[0];

    let maxSum = nums[0];
    let dp = nums[0];  // max sum ending at current position

    for (let i = 1; i < n; i++) {
        dp = Math.max(nums[i], dp + nums[i]);
        maxSum = Math.max(maxSum, dp);
    }

    return maxSum;
}

module.exports = maxSubArray;
