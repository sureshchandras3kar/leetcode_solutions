/**
 * Climb n stairs with Fibonacci variables approach (space-optimized).
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 */
function climbingStairsFibonacciVariables(n) {
    if (n <= 1) return 1;

    let prev2 = 1;  // dp[0]
    let prev1 = 1;  // dp[1]

    for (let i = 2; i <= n; i++) {
        const current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;
    }

    return prev1;
}

console.log(climbingStairsFibonacciVariables(3));  // 3
console.log(climbingStairsFibonacciVariables(4));  // 5
console.log(climbingStairsFibonacciVariables(5));  // 8
