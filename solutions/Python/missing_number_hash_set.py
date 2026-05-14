from typing import List


def missing_number(nums: List[int]) -> int:
    seen = set(nums)
    for value in range(len(nums) + 1):
        if value not in seen:
            return value
    return -1
