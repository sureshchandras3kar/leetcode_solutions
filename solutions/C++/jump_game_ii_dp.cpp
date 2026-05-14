#include <iostream>
#include <vector>
#include <algorithm>
#include <climits>

int jumpGameIIDP(std::vector<int>& nums) {
    /*
    Dynamic Programming approach: compute minimum jumps needed to reach each index.

    Intuition: dp[i] = minimum number of jumps to reach index i.
    For each index i, look back at all indices j where j + nums[j] >= i,
    meaning from j we can reach i in one jump. Take the minimum jumps[j] + 1.

    Time Complexity: O(n²) - for each index, we may check all previous indices
    Space Complexity: O(n) - dp array
    */
    if (nums.size() <= 1) {
        return 0;
    }

    int n = nums.size();
    std::vector<int> dp(n, INT_MAX);
    dp[0] = 0;  // Start at index 0 with 0 jumps

    for (int i = 1; i < n; ++i) {
        // Check all previous indices to see which can reach i
        for (int j = 0; j < i; ++j) {
            if (j + nums[j] >= i) {  // From index j, we can reach index i
                dp[i] = std::min(dp[i], dp[j] + 1);
                break;  // Optimization: once we find a j that reaches i, we can stop
            }
        }
    }

    return dp[n - 1];
}

int main() {
    std::vector<int> test1 = {2, 3, 1, 1, 4};
    std::cout << jumpGameIIDP(test1) << std::endl;  // 2

    std::vector<int> test2 = {2, 3, 0, 6, 9, 1, 2};
    std::cout << jumpGameIIDP(test2) << std::endl;  // 2

    std::vector<int> test3 = {10};
    std::cout << jumpGameIIDP(test3) << std::endl;  // 0

    std::vector<int> test4 = {1, 1, 1, 0};
    std::cout << jumpGameIIDP(test4) << std::endl;  // 3

    return 0;
}
