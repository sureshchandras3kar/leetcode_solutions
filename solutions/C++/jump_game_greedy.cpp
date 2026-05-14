#include <iostream>
#include <vector>
#include <algorithm>

bool canJumpGreedy(std::vector<int>& nums) {
    /*
     * Greedy approach: work backwards from the end to find the furthest index
     * we can reach. If we can reach index 0, we can reach the end.
     *
     * Time: O(n), Space: O(1)
     */
    int lastGoodIndex = (int)nums.size() - 1;
    for (int i = (int)nums.size() - 2; i >= 0; i--) {
        if (i + nums[i] >= lastGoodIndex) {
            lastGoodIndex = i;
        }
    }
    return lastGoodIndex == 0;
}

int main() {
    std::vector<int> nums1 = {2, 3, 1, 1, 4};
    std::vector<int> nums2 = {3, 2, 1, 0, 4};
    std::vector<int> nums3 = {0};
    std::vector<int> nums4 = {2, 0, 0};
    std::vector<int> nums5 = {0, 2, 3};

    std::cout << std::boolalpha;
    std::cout << canJumpGreedy(nums1) << std::endl;  // true
    std::cout << canJumpGreedy(nums2) << std::endl;  // false
    std::cout << canJumpGreedy(nums3) << std::endl;  // true
    std::cout << canJumpGreedy(nums4) << std::endl;  // true
    std::cout << canJumpGreedy(nums5) << std::endl;  // false

    return 0;
}
