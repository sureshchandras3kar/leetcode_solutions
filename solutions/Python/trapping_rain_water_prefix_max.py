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


# Approach: Prefix Max Arrays
# Build left_max[i] = max height from height[0] to height[i].
# Build right_max[i] = max height from height[i] to height[n-1].
# Water at i = max(0, min(left_max[i], right_max[i]) - height[i]).
# The minimum of both maxes is the effective wall height that bounds the water.

# Time Complexity: O(n) — three linear passes
# Space Complexity: O(n) — two extra arrays of size n

from typing import List


def trap(height: List[int]) -> int:
    n = len(height)
    if n == 0:
        return 0

    left_max = [0] * n
    right_max = [0] * n

    left_max[0] = height[0]
    for i in range(1, n):
        left_max[i] = max(left_max[i - 1], height[i])

    right_max[n - 1] = height[n - 1]
    for i in range(n - 2, -1, -1):
        right_max[i] = max(right_max[i + 1], height[i])

    water = 0
    for i in range(n):
        water += max(0, min(left_max[i], right_max[i]) - height[i])
    return water


print(trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]))  # 6
print(trap([4, 2, 0, 3, 2, 5]))                      # 9
