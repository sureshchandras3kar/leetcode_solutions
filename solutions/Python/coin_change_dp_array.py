from typing import List


def coin_change_dp_array(coins: List[int], amount: int) -> int:
    """
    Find minimum number of coins needed to make amount using DP array.

    Time Complexity: O(n * amount) where n = len(coins)
    Space Complexity: O(amount)

    dp[i] = minimum coins needed to make amount i
    For each amount, try all coins and take the minimum
    dp[i] = min(dp[i - coin] + 1) for all coins <= i
    """
    dp = [float('inf')] * (amount + 1)
    dp[0] = 0

    for i in range(1, amount + 1):
        for coin in coins:
            if coin <= i:
                dp[i] = min(dp[i], dp[i - coin] + 1)

    return dp[amount] if dp[amount] != float('inf') else -1


if __name__ == "__main__":
    print(coin_change_dp_array([1, 2, 5], 5))      # 1 (one 5-coin)
    print(coin_change_dp_array([2], 3))            # -1 (impossible)
    print(coin_change_dp_array([10], 10))          # 1 (one 10-coin)
    print(coin_change_dp_array([1, 3, 4], 6))      # 2 (two 3-coins or one 3-coin + three 1-coins)
