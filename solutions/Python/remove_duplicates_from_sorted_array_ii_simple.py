from typing import List


def removeDuplicates(nums: List[int]) -> int:
    """
    Simple Two Pointers Approach
    Allow at most 2 occurrences by checking if current element differs from element 2 positions back.

    Time Complexity: O(n)
    Space Complexity: O(1)
    """
    if not nums:
        return 0

    k = 0
    for i in range(len(nums)):
        # Always write first 2 elements, or if current differs from element 2 positions back
        if k < 2 or nums[i] != nums[k - 2]:
            nums[k] = nums[i]
            k += 1

    return k


# Test cases
print(removeDuplicates([1, 1, 1, 2, 2, 3]))  # 5, nums = [1, 1, 2, 2, 3, _]
print(removeDuplicates([0, 0, 1, 1, 1, 1, 2, 3, 3]))  # 7, nums = [0, 0, 1, 1, 2, 3, 3, _, _]
print(removeDuplicates([1]))  # 1, nums = [1]
print(removeDuplicates([1, 2]))  # 2, nums = [1, 2]
