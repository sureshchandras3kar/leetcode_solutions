#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

/**
 * Rob houses with maximum money using DP array approach.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */
int houseRobberDpArray(vector<int>& nums) {
    if (nums.empty()) return 0;
    if (nums.size() == 1) return nums[0];

    vector<int> dp(nums.size());
    dp[0] = nums[0];
    dp[1] = max(nums[0], nums[1]);

    for (int i = 2; i < nums.size(); i++) {
        dp[i] = max(nums[i] + dp[i - 2], dp[i - 1]);
    }

    return dp.back();
}

int main() {
    vector<int> test1 = {1, 2, 3, 1};
    cout << houseRobberDpArray(test1) << endl;  // 4

    vector<int> test2 = {2, 7, 9, 3, 1};
    cout << houseRobberDpArray(test2) << endl;  // 12

    return 0;
}
