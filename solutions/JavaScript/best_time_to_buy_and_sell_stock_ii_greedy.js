function bestTimeToBuyAndSellStockIIGreedy(prices) {
    let maxProfit = 0;
    for (let i = 1; i < prices.length; i++) {
        if (prices[i] > prices[i - 1]) {
            maxProfit += prices[i] - prices[i - 1];
        }
    }
    return maxProfit;
}

const prices1 = [7, 1, 5, 3, 6, 4];
console.log(bestTimeToBuyAndSellStockIIGreedy(prices1));  // 7

const prices2 = [1, 2, 3, 4, 5];
console.log(bestTimeToBuyAndSellStockIIGreedy(prices2));  // 4

const prices3 = [7, 6, 4, 3, 1];
console.log(bestTimeToBuyAndSellStockIIGreedy(prices3));  // 0
