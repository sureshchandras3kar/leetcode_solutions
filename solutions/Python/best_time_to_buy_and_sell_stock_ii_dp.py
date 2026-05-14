from typing import List


def best_time_to_buy_and_sell_stock_ii_dp(prices: List[int]) -> int:
    """
    Dynamic Programming approach: track holding and not holding states.
    Time: O(n), Space: O(1)
    """
    if not prices or len(prices) < 2:
        return 0

    hold = -prices[0]  # profit if holding stock after day 0
    no_hold = 0        # profit if not holding stock after day 0

    for i in range(1, len(prices)):
        # Either hold from before or buy today (reset from no_hold)
        hold = max(hold, no_hold - prices[i])
        # Either don't hold from before or sell today (from hold)
        no_hold = max(no_hold, hold + prices[i])

    return no_hold


print(best_time_to_buy_and_sell_stock_ii_dp([7, 1, 5, 3, 6, 4]))  # 7
print(best_time_to_buy_and_sell_stock_ii_dp([1, 2, 3, 4, 5]))  # 4
print(best_time_to_buy_and_sell_stock_ii_dp([7, 6, 4, 3, 1]))  # 0
