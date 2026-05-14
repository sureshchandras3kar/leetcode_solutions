// Approach: Brute Force
// Try every pair (i, j) where i < j. Profit = prices[j] - prices[i].
// Track the maximum profit across all pairs.
//
// Time Complexity: O(n²) — nested loops over all pairs
// Space Complexity: O(1)

#include <vector>
#include <algorithm>
#include <iostream>

using namespace std;

int maxProfit(vector<int>& prices) {
    int n = prices.size();
    int maxProfitVal = 0;

    for (int i = 0; i < n; i++) {
        for (int j = i + 1; j < n; j++) {
            int profit = prices[j] - prices[i];
            maxProfitVal = max(maxProfitVal, profit);
        }
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
