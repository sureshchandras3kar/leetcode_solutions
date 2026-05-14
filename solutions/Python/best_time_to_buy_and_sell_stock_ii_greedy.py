from typing import List


def best_time_to_buy_and_sell_stock_ii_greedy(prices: List[int]) -> int:
    """
    Greedy approach: capture every upward movement.
    Time: O(n), Space: O(1)
    """
    max_profit = 0
    for i in range(1, len(prices)):
        if prices[i] > prices[i - 1]:
            max_profit += prices[i] - prices[i - 1]
    return max_profit


print(best_time_to_buy_and_sell_stock_ii_greedy([7, 1, 5, 3, 6, 4]))  # 7
print(best_time_to_buy_and_sell_stock_ii_greedy([1, 2, 3, 4, 5]))  # 4
print(best_time_to_buy_and_sell_stock_ii_greedy([7, 6, 4, 3, 1]))  # 0
