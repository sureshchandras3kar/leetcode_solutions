from typing import List

def findMin(nums: List[int]) -> int:
    """Iterative approach: O(log n) time, O(1) space"""
    for i in range(len(nums) - 1):
        if nums[i] > nums[i + 1]:
            return nums[i + 1]
    return nums[0]
