/**
 * Climb n stairs with dp array approach.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */
function climbingStairsDpArray(n) {
    if (n <= 1) return 1;

    const dp = new Array(n + 1);
    dp[0] = 1;
    dp[1] = 1;

    for (let i = 2; i <= n; i++) {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    return dp[n];
}

console.log(climbingStairsDpArray(3));  // 3
console.log(climbingStairsDpArray(4));  // 5
console.log(climbingStairsDpArray(5));  // 8
