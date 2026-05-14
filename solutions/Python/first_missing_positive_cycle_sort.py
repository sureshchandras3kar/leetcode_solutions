from typing import List


def first_missing_positive(nums: List[int]) -> int:
    n = len(nums)
    i = 0

    # Place each number at its correct index (value v at index v-1)
    while i < n:
        j = nums[i] - 1
        if 1 <= nums[i] <= n and nums[j] != nums[i]:
            nums[i], nums[j] = nums[j], nums[i]
        else:
            i += 1

    # Find first index where value doesn't match expected
    for i in range(n):
        if nums[i] != i + 1:
            return i + 1

    return n + 1


print(first_missing_positive([1, 2, 0]))        # 3
print(first_missing_positive([3, 4, -1, 1]))    # 2
print(first_missing_positive([7, 8, 9, 11, 12]))  # 1
