from typing import List


def first_missing_positive(nums: List[int]) -> int:
    n = len(nums)

    # Phase 1: replace non-positives and values > n with n+1
    for i in range(n):
        if nums[i] <= 0 or nums[i] > n:
            nums[i] = n + 1

    # Phase 2: mark presence using negative signs
    for i in range(n):
        v = abs(nums[i])
        if 1 <= v <= n:
            nums[v - 1] = -abs(nums[v - 1])

    # Phase 3: find first unmarked index
    for i in range(n):
        if nums[i] > 0:
            return i + 1

    return n + 1


print(first_missing_positive([1, 2, 0]))        # 3
print(first_missing_positive([3, 4, -1, 1]))    # 2
print(first_missing_positive([7, 8, 9, 11, 12]))  # 1
