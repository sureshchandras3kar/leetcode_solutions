from typing import List


def change(amount: int, coins: List[int]) -> int:
    dp = [float('inf')] * (amount + 1)
    dp[0] = 0

    for coin in coins:
        for curr in range(coin, amount + 1):
            if dp[curr - coin] != float('inf'):
                dp[curr] = min(dp[curr], dp[curr - coin] + 1)

    return dp[amount] if dp[amount] != float('inf') else -1
