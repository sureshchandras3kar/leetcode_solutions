from typing import List

def findMin(nums: List[int]) -> int:
    """Binary search approach: O(log n) time, O(1) space"""
    left, right = 0, len(nums) - 1

    while left < right:
        mid = (left + right) // 2
        if nums[mid] > nums[right]:
            left = mid + 1
        else:
            right = mid

    return nums[left]
