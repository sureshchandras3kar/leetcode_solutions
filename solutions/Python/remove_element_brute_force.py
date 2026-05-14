"""
Remove all occurrences of a value in-place from an array and return the new length.

Approach: Brute Force
Scan the array. When an element equals val, remove it by shifting all subsequent elements
left by one position. Repeat until no more elements equal val are found.
Do not increment the index after removal so the shifted elements are re-examined.

Time Complexity: O(n²) — worst case when all elements equal val, each removal is O(n)
Space Complexity: O(1) — no extra data structures

Example 1:
Input: nums = [3,2,2,3], val = 3
Output: 2
Array after: [2,2,_,_]
Explanation: First 3 is removed, then second 3 is removed.

Example 2:
Input: nums = [0,1,2,2,3,0,4,2], val = 2
Output: 5
Array after: [0,1,4,0,3,_,_,_]
Explanation: Each 2 is removed by shifting subsequent elements left.
"""

from typing import List


def remove_element(nums: List[int], val: int) -> int:
    i = 0
    while i < len(nums):
        if nums[i] == val:
            nums.pop(i)  # Remove element and shift all elements after i one position left
        else:
            i += 1
    return len(nums)


# Test cases
print(remove_element([3, 2, 2, 3], val=3))  # 2
print(remove_element([0, 1, 2, 2, 3, 0, 4, 2], val=2))  # 5
print(remove_element([2], val=3))  # 1
print(remove_element([], val=0))  # 0
