#include <iostream>
#include <vector>
#include <algorithm>

// Approach: Prefix Max Arrays
// Build left_max[i] = max height from height[0] to height[i].
// Build right_max[i] = max height from height[i] to height[n-1].
// Water at i = max(0, min(left_max[i], right_max[i]) - height[i]).
// The minimum of both maxes is the effective wall height that bounds the water.
//
// Time Complexity: O(n) — three linear passes
// Space Complexity: O(n) — two extra arrays of size n

int trap(std::vector<int>& height) {
    int n = static_cast<int>(height.size());
    if (n == 0) return 0;

    std::vector<int> leftMax(n), rightMax(n);

    leftMax[0] = height[0];
    for (int i = 1; i < n; i++)
        leftMax[i] = std::max(leftMax[i - 1], height[i]);

    rightMax[n - 1] = height[n - 1];
    for (int i = n - 2; i >= 0; i--)
        rightMax[i] = std::max(rightMax[i + 1], height[i]);

    int water = 0;
    for (int i = 0; i < n; i++)
        water += std::max(0, std::min(leftMax[i], rightMax[i]) - height[i]);

    return water;
}

int main() {
    std::vector<int> h1 = {0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1};
    std::cout << trap(h1) << std::endl; // 6

    std::vector<int> h2 = {4, 2, 0, 3, 2, 5};
    std::cout << trap(h2) << std::endl; // 9

    return 0;
}
