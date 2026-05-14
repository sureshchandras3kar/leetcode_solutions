#include <iostream>
#include <vector>
#include <algorithm>

// Approach: Brute Force
// Try every pair (i, j). Area = min(height[i], height[j]) * (j - i). Track the maximum.
//
// Time Complexity: O(n^2)
// Space Complexity: O(1)

int maxAreaBruteForce(std::vector<int>& height) {
    int n = static_cast<int>(height.size());
    int maxWater = 0;
    for (int i = 0; i < n; i++) {
        for (int j = i + 1; j < n; j++) {
            int water = std::min(height[i], height[j]) * (j - i);
            maxWater = std::max(maxWater, water);
        }
    }
    return maxWater;
}

int main() {
    std::vector<int> height1 = {1, 8, 6, 2, 5, 4, 8, 3, 7};
    std::cout << maxAreaBruteForce(height1) << std::endl; // 49

    std::vector<int> height2 = {1, 1};
    std::cout << maxAreaBruteForce(height2) << std::endl; // 1

    return 0;
}
