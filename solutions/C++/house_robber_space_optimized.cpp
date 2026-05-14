#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

/**
 * Rob houses with maximum money using space-optimized approach.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 */
int houseRobberSpaceOptimized(vector<int>& nums) {
    if (nums.empty()) return 0;
    if (nums.size() == 1) return nums[0];

    int prev2 = nums[0];
    int prev1 = max(nums[0], nums[1]);

    for (int i = 2; i < nums.size(); i++) {
        int current = max(nums[i] + prev2, prev1);
        prev2 = prev1;
        prev1 = current;
    }

    return prev1;
}

int main() {
    vector<int> test1 = {1, 2, 3, 1};
    cout << houseRobberSpaceOptimized(test1) << endl;  // 4

    vector<int> test2 = {2, 7, 9, 3, 1};
    cout << houseRobberSpaceOptimized(test2) << endl;  // 12

    return 0;
}
