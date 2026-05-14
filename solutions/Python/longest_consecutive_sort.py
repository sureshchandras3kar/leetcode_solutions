from typing import List


def longest_consecutive(nums: List[int]) -> int:
    if not nums:
        return 0
    nums.sort()
    best = 1
    streak = 1
    for i in range(1, len(nums)):
        if nums[i] == nums[i - 1]:
            continue  # skip duplicate
        if nums[i] == nums[i - 1] + 1:
            streak += 1
            best = max(best, streak)
        else:
            streak = 1
    return best
