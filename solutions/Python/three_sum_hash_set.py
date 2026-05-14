from typing import List


def three_sum_hash_set(nums: List[int]) -> List[List[int]]:
    nums.sort()
    result = []
    n = len(nums)

    for i in range(n - 2):
        if i > 0 and nums[i] == nums[i - 1]:
            continue
        seen = set()
        j = i + 1
        while j < n:
            need = -(nums[i] + nums[j])
            if need in seen:
                triplet = [nums[i], need, nums[j]]
                if triplet not in result:
                    result.append(triplet)
            seen.add(nums[j])
            j += 1

    return result


print(three_sum_hash_set([-1, 0, 1, 2, -1, -4]))  # [[-1, -1, 2], [-1, 0, 1]]
print(three_sum_hash_set([0, 1, 1]))               # []
print(three_sum_hash_set([0, 0, 0]))               # [[0, 0, 0]]
