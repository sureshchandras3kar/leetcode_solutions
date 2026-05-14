# Approach: Single Pass with Min Price Tracking
# Track the minimum price seen so far and calculate the maximum profit at each price.
# When we see a new price, we check the profit if we sell at that price.
#
# Time Complexity: O(n) — single pass
# Space Complexity: O(1)

from typing import List


def max_profit(prices: List[int]) -> int:
    if not prices or len(prices) < 2:
        return 0

    min_price = prices[0]
    max_profit_val = 0

    for price in prices[1:]:
        profit = price - min_price
        max_profit_val = max(max_profit_val, profit)
        min_price = min(min_price, price)

    return max_profit_val


print(max_profit([7, 1, 5, 3, 6, 4]))  # 5
print(max_profit([7, 6, 4, 3, 1]))     # 0
print(max_profit([2, 4, 1, 7, 5, 11])) # 9
