#include <iostream>
#include <vector>
#include <algorithm>

bool canJumpDP(std::vector<int>& nums) {
    /*
     * Dynamic programming approach: forward pass tracking the furthest
     * reachable index. If we can ever reach the end index, return True.
     *
     * Time: O(n), Space: O(1)
     */
    int maxReach = 0;
    for (int i = 0; i < (int)nums.size(); i++) {
        if (i > maxReach) {
            return false;
        }
        maxReach = std::max(maxReach, i + nums[i]);
        if (maxReach >= (int)nums.size() - 1) {
            return true;
        }
    }
    return false;
}

int main() {
    std::vector<int> nums1 = {2, 3, 1, 1, 4};
    std::vector<int> nums2 = {3, 2, 1, 0, 4};
    std::vector<int> nums3 = {0};
    std::vector<int> nums4 = {2, 0, 0};
    std::vector<int> nums5 = {0, 2, 3};

    std::cout << std::boolalpha;
    std::cout << canJumpDP(nums1) << std::endl;  // true
    std::cout << canJumpDP(nums2) << std::endl;  // false
    std::cout << canJumpDP(nums3) << std::endl;  // true
    std::cout << canJumpDP(nums4) << std::endl;  // true
    std::cout << canJumpDP(nums5) << std::endl;  // false

    return 0;
}
