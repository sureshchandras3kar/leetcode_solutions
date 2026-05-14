from typing import List


def majority_element(nums: List[int]) -> int:
    candidate, count = nums[0], 1
    for num in nums[1:]:
        if count == 0:
            candidate, count = num, 1
        elif num == candidate:
            count += 1
        else:
            count -= 1
    return candidate


print(majority_element([3, 2, 3]))              # 3
print(majority_element([2, 2, 1, 1, 1, 2, 2])) # 2
