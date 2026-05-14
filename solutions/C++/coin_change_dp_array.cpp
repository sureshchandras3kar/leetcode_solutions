#include <iostream>
#include <vector>
#include <algorithm>
#include <climits>

using namespace std;

/**
 * Find minimum number of coins needed to make amount using DP array.
 *
 * Time Complexity: O(n * amount)
 * Space Complexity: O(amount)
 */
int coinChangeDP(vector<int>& coins, int amount) {
    vector<int> dp(amount + 1, INT_MAX);
    dp[0] = 0;

    for (int i = 1; i <= amount; i++) {
        for (int coin : coins) {
            if (coin <= i && dp[i - coin] != INT_MAX) {
                dp[i] = min(dp[i], dp[i - coin] + 1);
            }
        }
    }

    return dp[amount] == INT_MAX ? -1 : dp[amount];
}

int main() {
    vector<int> coins1 = {1, 2, 5};
    cout << coinChangeDP(coins1, 5) << endl;  // 1

    vector<int> coins2 = {2};
    cout << coinChangeDP(coins2, 3) << endl;  // -1

    return 0;
}
