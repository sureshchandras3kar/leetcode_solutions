from typing import List


def removeDuplicates(nums: List[int]) -> int:
    """
    Two Pointers with Counter Approach
    Allow each element to appear at most twice using explicit count tracking.

    Time Complexity: O(n)
    Space Complexity: O(1)
    """
    if not nums:
        return 0

    k = 1
    count = 1
    for i in range(1, len(nums)):
        if nums[i] != nums[i - 1]:
            # New element encountered, reset counter
            count = 1
            nums[k] = nums[i]
            k += 1
        elif count < 2:
            # Same element but less than 2 occurrences, allow it
            count += 1
            nums[k] = nums[i]
            k += 1
        # else: count == 2, skip this duplicate

    return k


# Test cases
print(removeDuplicates([1, 1, 1, 2, 2, 3]))  # 5, nums = [1, 1, 2, 2, 3, _]
print(removeDuplicates([0, 0, 1, 1, 1, 1, 2, 3, 3]))  # 7, nums = [0, 0, 1, 1, 2, 3, 3, _, _]
print(removeDuplicates([1]))  # 1, nums = [1]
print(removeDuplicates([1, 2]))  # 2, nums = [1, 2]
