from typing import List


def summary_ranges(nums: List[int]) -> List[str]:
    """
    Simulation approach: iterate through and build ranges as we go.
    Time: O(n), Space: O(1) excluding output
    """
    if not nums:
        return []

    result = []
    start = nums[0]

    for i in range(1, len(nums)):
        # If current number is not consecutive, finalize the range
        if nums[i] != nums[i - 1] + 1:
            if start == nums[i - 1]:
                result.append(str(start))
            else:
                result.append(f"{start}->{nums[i - 1]}")
            start = nums[i]

    # Add the last range
    if start == nums[-1]:
        result.append(str(start))
    else:
        result.append(f"{start}->{nums[-1]}")

    return result


print(summary_ranges([0, 1, 2, 4, 5, 7]))  # ["0->2","4->5","7"]
print(summary_ranges([0, 2, 3, 4, 6, 8, 9]))  # ["0","2->4","6","8->9"]
