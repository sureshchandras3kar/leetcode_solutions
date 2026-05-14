#include <iostream>
#include <vector>

int bestTimeToBuyAndSellStockIIGreedy(std::vector<int>& prices) {
    int maxProfit = 0;
    for (int i = 1; i < (int)prices.size(); i++) {
        if (prices[i] > prices[i - 1]) {
            maxProfit += prices[i] - prices[i - 1];
        }
    }
    return maxProfit;
}

int main() {
    std::vector<int> prices1 = {7, 1, 5, 3, 6, 4};
    std::cout << bestTimeToBuyAndSellStockIIGreedy(prices1) << std::endl;  // 7

    std::vector<int> prices2 = {1, 2, 3, 4, 5};
    std::cout << bestTimeToBuyAndSellStockIIGreedy(prices2) << std::endl;  // 4

    std::vector<int> prices3 = {7, 6, 4, 3, 1};
    std::cout << bestTimeToBuyAndSellStockIIGreedy(prices3) << std::endl;  // 0

    return 0;
}
