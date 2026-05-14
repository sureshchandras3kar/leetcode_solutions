#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

/**
 * Find length of longest increasing subsequence using DP O(n²).
 *
 * Time Complexity: O(n²)
 * Space Complexity: O(n)
 */
int lisDP(vector<int>& nums) {
    if (nums.empty()) return 0;

    int n = nums.size();
    vector<int> dp(n, 1);

    for (int i = 1; i < n; i++) {
        for (int j = 0; j < i; j++) {
            if (nums[j] < nums[i]) {
                dp[i] = max(dp[i], dp[j] + 1);
            }
        }
    }

    return *max_element(dp.begin(), dp.end());
}

int main() {
    vector<int> test1 = {10, 9, 2, 5, 3, 7, 101, 18};
    cout << lisDP(test1) << endl;  // 4
    return 0;
}
