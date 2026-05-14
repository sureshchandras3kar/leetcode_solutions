from typing import List


def summary_ranges(nums: List[int]) -> List[str]:
    """
    Two-pointer approach: explicitly track start and end pointers.
    Time: O(n), Space: O(1) excluding output
    """
    if not nums:
        return []

    result = []
    left = 0

    for right in range(len(nums)):
        # Check if next element breaks the sequence
        if right == len(nums) - 1 or nums[right + 1] != nums[right] + 1:
            if left == right:
                result.append(str(nums[left]))
            else:
                result.append(f"{nums[left]}->{nums[right]}")
            left = right + 1

    return result


print(summary_ranges([0, 1, 2, 4, 5, 7]))  # ["0->2","4->5","7"]
print(summary_ranges([0, 2, 3, 4, 6, 8, 9]))  # ["0","2->4","6","8->9"]
