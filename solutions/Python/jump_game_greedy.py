from typing import List


def can_jump_greedy(nums: List[int]) -> bool:
    """
    Greedy approach: work backwards from the end to find the furthest index
    we can reach. If we can reach index 0, we can reach the end.

    Time: O(n), Space: O(1)
    """
    last_good_index = len(nums) - 1
    for i in range(len(nums) - 2, -1, -1):
        if i + nums[i] >= last_good_index:
            last_good_index = i
    return last_good_index == 0


# Test cases
print(can_jump_greedy([2, 3, 1, 1, 4]))  # True
print(can_jump_greedy([3, 2, 1, 0, 4]))  # False
print(can_jump_greedy([0]))  # True
print(can_jump_greedy([2, 0, 0]))  # True
print(can_jump_greedy([0, 2, 3]))  # False
