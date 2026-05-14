#include <iostream>
#include <vector>
#include <queue>
#include <unordered_set>

using namespace std;

/**
 * Find minimum number of coins needed to make amount using BFS.
 *
 * Time Complexity: O(n * amount)
 * Space Complexity: O(amount)
 */
int coinChangeBFS(vector<int>& coins, int amount) {
    if (amount == 0) return 0;

    queue<pair<int, int>> q;  // (remaining_amount, num_coins)
    unordered_set<int> visited;

    q.push({amount, 0});
    visited.insert(amount);

    while (!q.empty()) {
        auto [curr_amount, num_coins] = q.front();
        q.pop();

        for (int coin : coins) {
            int next_amount = curr_amount - coin;
            if (next_amount == 0) {
                return num_coins + 1;
            }
            if (next_amount > 0 && visited.find(next_amount) == visited.end()) {
                visited.insert(next_amount);
                q.push({next_amount, num_coins + 1});
            }
        }
    }

    return -1;
}

int main() {
    vector<int> coins1 = {1, 2, 5};
    cout << coinChangeBFS(coins1, 5) << endl;  // 1

    vector<int> coins2 = {2};
    cout << coinChangeBFS(coins2, 3) << endl;  // -1

    return 0;
}
