/**
 * Dynamic Programming approach: compute minimum jumps needed to reach each index.
 *
 * Intuition: dp[i] = minimum number of jumps to reach index i.
 * For each index i, look back at all indices j where j + nums[j] >= i,
 * meaning from j we can reach i in one jump. Take the minimum jumps[j] + 1.
 *
 * Time Complexity: O(n²) - for each index, we may check all previous indices
 * Space Complexity: O(n) - dp array
 *
 * @param {number[]} nums - The array of jump values
 * @return {number} - Minimum number of jumps to reach the last index
 */
function jumpGameIIDP(nums) {
    if (nums.length <= 1) {
        return 0;
    }

    const n = nums.length;
    const dp = Array(n).fill(Infinity);
    dp[0] = 0;  // Start at index 0 with 0 jumps

    for (let i = 1; i < n; i++) {
        // Check all previous indices to see which can reach i
        for (let j = 0; j < i; j++) {
            if (j + nums[j] >= i) {  // From index j, we can reach index i
                dp[i] = Math.min(dp[i], dp[j] + 1);
                break;  // Optimization: once we find a j that reaches i, we can stop
            }
        }
    }

    return dp[n - 1];
}

// Test cases
console.log(jumpGameIIDP([2, 3, 1, 1, 4]));        // 2
console.log(jumpGameIIDP([2, 3, 0, 6, 9, 1, 2]));  // 3
console.log(jumpGameIIDP([10]));                   // 0
console.log(jumpGameIIDP([1, 1, 1, 0]));           // 3

export default jumpGameIIDP;
