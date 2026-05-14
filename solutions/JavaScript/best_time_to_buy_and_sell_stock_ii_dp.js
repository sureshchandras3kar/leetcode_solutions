function bestTimeToBuyAndSellStockIIDP(prices) {
    if (!prices || prices.length < 2) {
        return 0;
    }

    let hold = -prices[0];      // profit if holding stock after day 0
    let noHold = 0;             // profit if not holding stock after day 0

    for (let i = 1; i < prices.length; i++) {
        // Either hold from before or buy today (reset from noHold)
        hold = Math.max(hold, noHold - prices[i]);
        // Either don't hold from before or sell today (from hold)
        noHold = Math.max(noHold, hold + prices[i]);
    }

    return noHold;
}

const prices1 = [7, 1, 5, 3, 6, 4];
console.log(bestTimeToBuyAndSellStockIIDP(prices1));  // 7

const prices2 = [1, 2, 3, 4, 5];
console.log(bestTimeToBuyAndSellStockIIDP(prices2));  // 4

const prices3 = [7, 6, 4, 3, 1];
console.log(bestTimeToBuyAndSellStockIIDP(prices3));  // 0
