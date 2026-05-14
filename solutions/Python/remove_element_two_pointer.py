"""
Remove all occurrences of a value in-place from an array and return the new length.

Approach: Two Pointers
Use a 'write' pointer to place non-val elements and a 'read' pointer to scan the array.
When read finds a non-val element, copy it to the write position and advance both.
When read finds a val element, skip it and only advance read.

Time Complexity: O(n) — single pass through the array
Space Complexity: O(1) — no extra data structures

Example 1:
Input: nums = [3,2,2,3], val = 3
Output: 2
Array after: [2,2,_,_]
Explanation: The first 2 elements are [2, 2]; elements after position 2 are ignored.

Example 2:
Input: nums = [0,1,2,2,3,0,4,2], val = 2
Output: 5
Array after: [0,1,4,0,3,_,_,_]
Explanation: The first 5 elements contain no 2.
"""

from typing import List


def remove_element(nums: List[int], val: int) -> int:
    write = 0
    for read in range(len(nums)):
        if nums[read] != val:
            nums[write] = nums[read]
            write += 1
    return write


# Test cases
print(remove_element([3, 2, 2, 3], val=3))  # 2
print(remove_element([0, 1, 2, 2, 3, 0, 4, 2], val=2))  # 5
print(remove_element([2], val=3))  # 1
print(remove_element([], val=0))  # 0
