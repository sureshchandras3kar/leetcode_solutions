"""
You are given an integer array height of length n. There are n vertical lines drawn
such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

Find two lines that together with the x-axis form a container, such that the container
contains the most water. Return the maximum amount of water a container can store.

Notice that you may not slant the container.

Example 1:
Input: height = [1,8,6,2,5,4,8,3,7]
Output: 49
Explanation: The vertical lines are drawn at the above positions. The maximum area is
obtained between index 1 and index 8: min(8,7) * (8-1) = 49.

Example 2:
Input: height = [1,1]
Output: 1
"""


# Approach: Two Pointers
# Place one pointer at the start and one at the end. The area is min(height[left], height[right]) * (right - left).
# Always move the pointer with the shorter height inward — the shorter wall is the bottleneck,
# so moving the taller pointer can only decrease or maintain the area.

# Time Complexity: O(n) — single pass
# Space Complexity: O(1)

from typing import List


def max_area(height: List[int]) -> int:
    left, right = 0, len(height) - 1
    max_water = 0
    while left < right:
        water = min(height[left], height[right]) * (right - left)
        max_water = max(max_water, water)
        if height[left] <= height[right]:
            left += 1
        else:
            right -= 1
    return max_water


print(max_area([1, 8, 6, 2, 5, 4, 8, 3, 7]))  # 49
print(max_area([1, 1]))                         # 1
