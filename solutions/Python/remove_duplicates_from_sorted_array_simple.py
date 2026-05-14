from typing import List


def removeDuplicates(nums: List[int]) -> int:
    """
    Simple Pass Approach
    Remove duplicates by iterating and comparing consecutive elements.

    Time Complexity: O(n)
    Space Complexity: O(1)
    """
    if not nums:
        return 0

    write_idx = 0
    for read_idx in range(1, len(nums)):
        if nums[read_idx] != nums[write_idx]:
            write_idx += 1
            nums[write_idx] = nums[read_idx]

    return write_idx + 1


# Test cases
print(removeDuplicates([1, 1, 2]))  # 2, nums = [1, 2, _]
print(removeDuplicates([0, 0, 1, 1, 1, 2, 2, 3, 3, 4]))  # 5, nums = [0, 1, 2, 3, 4, _, _, _, _, _]
