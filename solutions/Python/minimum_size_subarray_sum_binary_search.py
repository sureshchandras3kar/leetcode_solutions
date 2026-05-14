from typing import List
import bisect


def minimum_size_subarray_sum_binary_search(target: int, nums: List[int]) -> int:
    prefix = [0]
    for num in nums:
        prefix.append(prefix[-1] + num)

    min_length = float('inf')

    for right in range(1, len(prefix)):
        needed = prefix[right] - target
        left = bisect.bisect_right(prefix, needed, 0, right) - 1

        if left >= 0 and left < right:
            min_length = min(min_length, right - left)

    return min_length if min_length != float('inf') else 0

print(minimum_size_subarray_sum_binary_search(7, [2, 3, 1, 2, 4, 3]))  # 2
