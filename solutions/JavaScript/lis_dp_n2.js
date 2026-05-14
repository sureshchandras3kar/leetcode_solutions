/**
 * Find length of longest increasing subsequence using DP O(n²).
 *
 * Time Complexity: O(n²)
 * Space Complexity: O(n)
 */
function lisDP(nums) {
    if (nums.length === 0) return 0;

    const dp = new Array(nums.length).fill(1);

    for (let i = 1; i < nums.length; i++) {
        for (let j = 0; j < i; j++) {
            if (nums[j] < nums[i]) {
                dp[i] = Math.max(dp[i], dp[j] + 1);
            }
        }
    }

    return Math.max(...dp);
}

console.log(lisDP([10, 9, 2, 5, 3, 7, 101, 18]));  // 4
