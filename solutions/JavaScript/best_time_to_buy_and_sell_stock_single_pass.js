// Approach: Single Pass with Min Price Tracking
// Track the minimum price seen so far and calculate the maximum profit at each price.
// When we see a new price, we check the profit if we sell at that price.
//
// Time Complexity: O(n) — single pass
// Space Complexity: O(1)

function maxProfit(prices) {
    if (!prices || prices.length < 2) {
        return 0;
    }

    let minPrice = prices[0];
    let maxProfitVal = 0;

    for (let i = 1; i < prices.length; i++) {
        const price = prices[i];
        const profit = price - minPrice;
        maxProfitVal = Math.max(maxProfitVal, profit);
        minPrice = Math.min(minPrice, price);
    }

    return maxProfitVal;
}


console.log(maxProfit([7, 1, 5, 3, 6, 4]));  // 5
console.log(maxProfit([7, 6, 4, 3, 1]));     // 0
console.log(maxProfit([2, 4, 1, 7, 5, 11])); // 9
