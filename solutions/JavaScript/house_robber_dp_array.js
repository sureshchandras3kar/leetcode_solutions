/**
 * Rob houses with maximum money using DP array approach.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */
function houseRobberDpArray(nums) {
    if (nums.length === 0) return 0;
    if (nums.length === 1) return nums[0];

    const dp = new Array(nums.length);
    dp[0] = nums[0];
    dp[1] = Math.max(nums[0], nums[1]);

    for (let i = 2; i < nums.length; i++) {
        dp[i] = Math.max(nums[i] + dp[i - 2], dp[i - 1]);
    }

    return dp[nums.length - 1];
}

console.log(houseRobberDpArray([1, 2, 3, 1]));      // 4
console.log(houseRobberDpArray([2, 7, 9, 3, 1]));   // 12
