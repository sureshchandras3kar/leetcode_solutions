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


# Approach: Two Pointers
# Use left and right pointers, tracking max_left and max_right seen so far.
# Process whichever side has the smaller max — that side's max is the bottleneck.
# water at current position = max_on_that_side - height[current]
# Update max before adding water, then advance the pointer.

# Time Complexity: O(n) — single pass
# Space Complexity: O(1) — four variables only

from typing import List


def trap(height: List[int]) -> int:
    left, right = 0, len(height) - 1
    max_left, max_right = 0, 0
    water = 0
    while left < right:
        if max_left <= max_right:
            max_left = max(max_left, height[left])
            water += max_left - height[left]
            left += 1
        else:
            max_right = max(max_right, height[right])
            water += max_right - height[right]
            right -= 1
    return water


print(trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]))  # 6
print(trap([4, 2, 0, 3, 2, 5]))                      # 9
