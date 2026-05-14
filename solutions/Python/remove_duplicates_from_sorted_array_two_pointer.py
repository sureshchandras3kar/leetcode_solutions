from typing import List


def removeDuplicates(nums: List[int]) -> int:
    """
    Two-Pointer In-Place Approach
    Remove duplicates from sorted array in-place and return the length of the new array.

    Time Complexity: O(n)
    Space Complexity: O(1)
    """
    if not nums:
        return 0

    k = 1  # First element is always unique
    for i in range(1, len(nums)):
        if nums[i] != nums[i - 1]:
            nums[k] = nums[i]
            k += 1

    return k


# Test cases
print(removeDuplicates([1, 1, 2]))  # 2, nums = [1, 2, _]
print(removeDuplicates([0, 0, 1, 1, 1, 2, 2, 3, 3, 4]))  # 5, nums = [0, 1, 2, 3, 4, _, _, _, _, _]
