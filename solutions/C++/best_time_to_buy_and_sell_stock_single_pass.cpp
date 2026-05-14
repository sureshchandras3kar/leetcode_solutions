// Approach: Single Pass with Min Price Tracking
// Track the minimum price seen so far and calculate the maximum profit at each price.
// When we see a new price, we check the profit if we sell at that price.
//
// Time Complexity: O(n) — single pass
// Space Complexity: O(1)

#include <vector>
#include <algorithm>
#include <iostream>

using namespace std;

int maxProfit(vector<int>& prices) {
    if (prices.empty() || prices.size() < 2) {
        return 0;
    }

    int minPrice = prices[0];
    int maxProfitVal = 0;

    for (int i = 1; i < prices.size(); i++) {
        int profit = prices[i] - minPrice;
        maxProfitVal = max(maxProfitVal, profit);
        minPrice = min(minPrice, prices[i]);
    }

    return maxProfitVal;
}


int main() {
    vector<int> prices1 = {7, 1, 5, 3, 6, 4};
    cout << maxProfit(prices1) << endl;  // 5

    vector<int> prices2 = {7, 6, 4, 3, 1};
    cout << maxProfit(prices2) << endl;  // 0

    vector<int> prices3 = {2, 4, 1, 7, 5, 11};
    cout << maxProfit(prices3) << endl;  // 9

    return 0;
}
