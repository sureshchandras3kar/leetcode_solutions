from typing import List


def jump_game_ii_greedy(nums: List[int]) -> int:
    """
    Greedy approach: track the farthest reachable index and jump count.

    Intuition: We maintain the range [current_start, current_end] that can be reached
    with the current number of jumps. When we exhaust this range, we increment jumps
    and expand to [current_end + 1, farthest], which is reachable with one more jump.

    Time Complexity: O(n) - single pass through array
    Space Complexity: O(1) - only use constant extra space
    """
    if len(nums) <= 1:
        return 0

    jumps = 0
    current_end = 0  # End of range reachable with current number of jumps
    farthest = 0     # Farthest index reachable so far

    for i in range(len(nums) - 1):  # No need to check the last index
        # Update the farthest index we can reach
        farthest = max(farthest, i + nums[i])

        # If we've reached the end of current jump range, we must jump
        if i == current_end:
            jumps += 1
            current_end = farthest

            # Optimization: if we can reach the end, no need to continue
            if current_end >= len(nums) - 1:
                break

    return jumps


# Test cases
print(jump_game_ii_greedy([2, 3, 1, 1, 4]))     # 2 (0->1->4)
print(jump_game_ii_greedy([2, 3, 0, 6, 9, 1, 2]))  # 3 (0->1->3/4->6)
print(jump_game_ii_greedy([10]))                # 0 (already at end)
print(jump_game_ii_greedy([1, 1, 1, 0]))        # 3 (must jump 1 step at a time)
