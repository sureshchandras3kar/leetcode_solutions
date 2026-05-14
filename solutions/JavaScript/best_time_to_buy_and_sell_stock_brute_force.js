// Approach: Brute Force
// Try every pair (i, j) where i < j. Profit = prices[j] - prices[i].
// Track the maximum profit across all pairs.
//
// Time Complexity: O(n²) — nested loops over all pairs
// Space Complexity: O(1)

function maxProfit(prices) {
    const n = prices.length;
    let maxProfitVal = 0;

    for (let i = 0; i < n; i++) {
        for (let j = i + 1; j < n; j++) {
            const profit = prices[j] - prices[i];
            maxProfitVal = Math.max(maxProfitVal, profit);
        }
    }

    return maxProfitVal;
}


console.log(maxProfit([7, 1, 5, 3, 6, 4]));  // 5
console.log(maxProfit([7, 6, 4, 3, 1]));     // 0
console.log(maxProfit([2, 4, 1, 7, 5, 11])); // 9
