"""
Given n non-negative integers representing an elevation map where the width of each
bar is 1, compute how much water it can trap after raining.

Example 1:
Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6

Example 2:
Input: height = [4,2,0,3,2,5]
Output: 9
"""


# Approach: Monotonic Stack
# Maintain a stack of indices with decreasing heights (a monotonic decreasing stack).
# When height[i] > height[stack.top()], we found a right wall — the pool can be filled.
# Pop the bottom index, compute water between the new stack top (left wall) and i (right wall).
# Think horizontally: each pop fills one horizontal layer of trapped water.

# Time Complexity: O(n) — each bar is pushed and popped at most once
# Space Complexity: O(n) — stack stores indices

from typing import List


def trap(height: List[int]) -> int:
    stack: List[int] = []
    water = 0
    for i in range(len(height)):
        while stack and height[i] > height[stack[-1]]:
            bottom = stack.pop()
            if not stack:
                break
            left = stack[-1]
            width = i - left - 1
            bounded_height = min(height[left], height[i]) - height[bottom]
            water += bounded_height * width
        stack.append(i)
    return water


print(trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]))  # 6
print(trap([4, 2, 0, 3, 2, 5]))                      # 9
