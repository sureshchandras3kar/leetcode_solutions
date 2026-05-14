#include <iostream>
#include <vector>
#include <algorithm>

// Approach: Two Pointers
// Use left and right pointers, tracking max_left and max_right seen so far.
// Process whichever side has the smaller max — that side's max is the bottleneck.
// water at current position = max_on_that_side - height[current]
// Update max before adding water, then advance the pointer.
//
// Time Complexity: O(n) — single pass
// Space Complexity: O(1) — four variables only

int trap(std::vector<int>& height) {
    int left = 0;
    int right = static_cast<int>(height.size()) - 1;
    int maxLeft = 0, maxRight = 0;
    int water = 0;
    while (left < right) {
        if (maxLeft <= maxRight) {
            maxLeft = std::max(maxLeft, height[left]);
            water += maxLeft - height[left];
            left++;
        } else {
            maxRight = std::max(maxRight, height[right]);
            water += maxRight - height[right];
            right--;
        }
    }
    return water;
}

int main() {
    std::vector<int> h1 = {0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1};
    std::cout << trap(h1) << std::endl; // 6

    std::vector<int> h2 = {4, 2, 0, 3, 2, 5};
    std::cout << trap(h2) << std::endl; // 9

    return 0;
}
