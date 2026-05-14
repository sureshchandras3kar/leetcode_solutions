/**
 * Find minimum number of coins needed to make amount using DP array.
 *
 * Time Complexity: O(n * amount)
 * Space Complexity: O(amount)
 */
function coinChangeDP(coins, amount) {
    const dp = new Array(amount + 1).fill(Infinity);
    dp[0] = 0;

    for (let i = 1; i <= amount; i++) {
        for (const coin of coins) {
            if (coin <= i) {
                dp[i] = Math.min(dp[i], dp[i - coin] + 1);
            }
        }
    }

    return dp[amount] === Infinity ? -1 : dp[amount];
}

console.log(coinChangeDP([1, 2, 5], 5));   // 1
console.log(coinChangeDP([2], 3));        // -1
