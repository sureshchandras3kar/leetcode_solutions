#include <iostream>
#include <vector>
#include <algorithm>

// Approach: Two Pointers
// Place one pointer at the start and one at the end.
// Area = min(height[left], height[right]) * (right - left).
// Move the pointer with the shorter height inward — the shorter wall is the bottleneck.
//
// Time Complexity: O(n)
// Space Complexity: O(1)

int maxArea(std::vector<int>& height) {
    int left = 0;
    int right = static_cast<int>(height.size()) - 1;
    int maxWater = 0;
    while (left < right) {
        int water = std::min(height[left], height[right]) * (right - left);
        maxWater = std::max(maxWater, water);
        if (height[left] <= height[right]) {
            left++;
        } else {
            right--;
        }
    }
    return maxWater;
}

int main() {
    std::vector<int> height1 = {1, 8, 6, 2, 5, 4, 8, 3, 7};
    std::cout << maxArea(height1) << std::endl; // 49

    std::vector<int> height2 = {1, 1};
    std::cout << maxArea(height2) << std::endl; // 1

    return 0;
}
