from typing import List


def can_jump_dp(nums: List[int]) -> bool:
    """
    Dynamic programming approach: forward pass tracking the furthest
    reachable index. If we can ever reach the end index, return True.

    Time: O(n), Space: O(1)
    """
    max_reach = 0
    for i in range(len(nums)):
        if i > max_reach:
            return False
        max_reach = max(max_reach, i + nums[i])
        if max_reach >= len(nums) - 1:
            return True
    return False


# Test cases
print(can_jump_dp([2, 3, 1, 1, 4]))  # True
print(can_jump_dp([3, 2, 1, 0, 4]))  # False
print(can_jump_dp([0]))  # True
print(can_jump_dp([2, 0, 0]))  # True
print(can_jump_dp([0, 2, 3]))  # False
