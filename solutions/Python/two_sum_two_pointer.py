from typing import List


def two_sum_two_pointer(nums: List[int], target: int) -> List[int]:
    nums_sorted = sorted(nums)
    left, right = 0, len(nums_sorted) - 1
    while left < right:
        current_sum = nums_sorted[left] + nums_sorted[right]
        if current_sum == target:
            return [nums_sorted[left], nums_sorted[right]]
        elif current_sum < target:
            left += 1
        else:
            right -= 1
    return []

print(two_sum_two_pointer([2, 7, 11, 15], target=9))   # [2, 7]
print(two_sum_two_pointer([3, 2, 4], target=6))         # [2, 4]
print(two_sum_two_pointer([3, 3], target=6))            # [3, 3]
