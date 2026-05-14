from typing import List


def subarray_sum_brute_force(nums: List[int], k: int) -> int:
    result = 0
    for i in range(len(nums)):
        total = 0
        for j in range(i, len(nums)):
            total += nums[j]
            if total == k:
                result += 1
    return result

print(subarray_sum_brute_force([1, 1, 1], k=2))  # 2
print(subarray_sum_brute_force([1, 2, 3], k=3))  # 2
