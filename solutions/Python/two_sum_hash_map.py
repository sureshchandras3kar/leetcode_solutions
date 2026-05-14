from typing import List


def two_sum_hash_map(nums: List[int], target: int) -> List[int]:
    seen = {}
    for i, num in enumerate(nums):
        complement = target - num
        if complement in seen:
            return [seen[complement], i]
        seen[num] = i
    return []

print(two_sum_hash_map([2, 7, 11, 15], target=9))  # [0, 1]
