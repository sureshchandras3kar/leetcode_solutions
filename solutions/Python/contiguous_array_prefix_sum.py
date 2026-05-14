from typing import List


def find_max_length(nums: List[int]) -> int:
    seen = {0: -1}
    prefix = 0
    best = 0
    for i, num in enumerate(nums):
        prefix += 1 if num == 1 else -1
        if prefix in seen:
            best = max(best, i - seen[prefix])
        else:
            seen[prefix] = i
    return best

print(find_max_length([0, 1]))      # 2
print(find_max_length([0, 1, 0]))   # 2
print(find_max_length([0, 1, 0, 1]))  # 4
