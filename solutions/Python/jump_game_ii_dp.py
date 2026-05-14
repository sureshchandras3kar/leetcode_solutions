from typing import List


def jump_game_ii_dp(nums: List[int]) -> int:
    """
    Dynamic Programming approach: compute minimum jumps needed to reach each index.

    Intuition: dp[i] = minimum number of jumps to reach index i.
    For each index i, look back at all indices j where j + nums[j] >= i,
    meaning from j we can reach i in one jump. Take the minimum jumps[j] + 1.

    Time Complexity: O(n²) - for each index, we may check all previous indices
    Space Complexity: O(n) - dp array

    This approach is less efficient than greedy but illustrates the classic DP pattern.
    """
    if len(nums) <= 1:
        return 0

    n = len(nums)
    # dp[i] = minimum jumps needed to reach index i
    dp = [float('inf')] * n
    dp[0] = 0  # Start at index 0 with 0 jumps

    for i in range(1, n):
        # Check all previous indices to see which can reach i
        for j in range(i):
            if j + nums[j] >= i:  # From index j, we can reach index i
                dp[i] = min(dp[i], dp[j] + 1)
                break  # Optimization: since we want minimum, once we find a j that reaches i, we can stop

    return dp[n - 1]


# Test cases
print(jump_game_ii_dp([2, 3, 1, 1, 4]))        # 2
print(jump_game_ii_dp([2, 3, 0, 6, 9, 1, 2]))  # 3
print(jump_game_ii_dp([10]))                   # 0
print(jump_game_ii_dp([1, 1, 1, 0]))           # 3
