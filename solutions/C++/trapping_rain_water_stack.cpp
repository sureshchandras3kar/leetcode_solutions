#include <iostream>
#include <vector>
#include <stack>
#include <algorithm>

// Approach: Monotonic Stack
// Maintain a stack of indices with decreasing heights (a monotonic decreasing stack).
// When height[i] > height[stack.top()], we found a right wall — the pool can be filled.
// Pop the bottom index, compute water between the new stack top (left wall) and i (right wall).
// Think horizontally: each pop fills one horizontal layer of trapped water.
//
// Time Complexity: O(n) — each bar is pushed and popped at most once
// Space Complexity: O(n) — stack stores indices

int trap(std::vector<int>& height) {
    std::stack<int> stk;
    int water = 0;
    for (int i = 0; i < static_cast<int>(height.size()); i++) {
        while (!stk.empty() && height[i] > height[stk.top()]) {
            int bottom = stk.top();
            stk.pop();
            if (stk.empty()) break;
            int left = stk.top();
            int width = i - left - 1;
            int boundedHeight = std::min(height[left], height[i]) - height[bottom];
            water += boundedHeight * width;
        }
        stk.push(i);
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
