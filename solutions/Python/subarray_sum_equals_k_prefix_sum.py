from typing import List


def subarray_sum_prefix_sum(nums: List[int], k: int) -> int:
    count = {0: 1}
    prefix = 0
    result = 0
    for num in nums:
        prefix += num
        result += count.get(prefix - k, 0)
        count[prefix] = count.get(prefix, 0) + 1
    return result

print(subarray_sum_prefix_sum([1, 1, 1], k=2))  # 2
print(subarray_sum_prefix_sum([1, 2, 3], k=3))  # 2
