#include <iostream>
#include <vector>
#include <algorithm>

int bestTimeToBuyAndSellStockIIDP(std::vector<int>& prices) {
    if (prices.empty() || prices.size() < 2) {
        return 0;
    }

    int hold = -prices[0];      // profit if holding stock after day 0
    int noHold = 0;             // profit if not holding stock after day 0

    for (int i = 1; i < (int)prices.size(); i++) {
        // Either hold from before or buy today (reset from noHold)
        hold = std::max(hold, noHold - prices[i]);
        // Either don't hold from before or sell today (from hold)
        noHold = std::max(noHold, hold + prices[i]);
    }

    return noHold;
}

int main() {
    std::vector<int> prices1 = {7, 1, 5, 3, 6, 4};
    std::cout << bestTimeToBuyAndSellStockIIDP(prices1) << std::endl;  // 7

    std::vector<int> prices2 = {1, 2, 3, 4, 5};
    std::cout << bestTimeToBuyAndSellStockIIDP(prices2) << std::endl;  // 4

    std::vector<int> prices3 = {7, 6, 4, 3, 1};
    std::cout << bestTimeToBuyAndSellStockIIDP(prices3) << std::endl;  // 0

    return 0;
}
