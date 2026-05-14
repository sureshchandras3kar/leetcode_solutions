# Approach: Brute Force
# Try every pair (i, j) where i < j. Profit = prices[j] - prices[i].
# Track the maximum profit across all pairs.
#
# Time Complexity: O(n²) — nested loops over all pairs
# Space Complexity: O(1)

from typing import List


def max_profit(prices: List[int]) -> int:
    n = len(prices)
    max_profit_val = 0

    for i in range(n):
        for j in range(i + 1, n):
            profit = prices[j] - prices[i]
            max_profit_val = max(max_profit_val, profit)

    return max_profit_val


print(max_profit([7, 1, 5, 3, 6, 4]))  # 5
print(max_profit([7, 6, 4, 3, 1]))     # 0
print(max_profit([2, 4, 1, 7, 5, 11])) # 9
