from typing import List


def climbing_stairs_dp_array(n: int) -> int:
    """
    Climb n stairs with dp array approach.

    Time Complexity: O(n)
    Space Complexity: O(n)

    At each step i, we can either:
    - Come from step i-1 and take 1 step
    - Come from step i-2 and take 2 steps
    So dp[i] = dp[i-1] + dp[i-2]
    """
    if n <= 1:
        return 1

    dp = [0] * (n + 1)
    dp[0] = 1
    dp[1] = 1

    for i in range(2, n + 1):
        dp[i] = dp[i - 1] + dp[i - 2]

    return dp[n]


if __name__ == "__main__":
    print(climbing_stairs_dp_array(3))   # 3 (1+1+1, 1+2, 2+1)
    print(climbing_stairs_dp_array(4))   # 5
    print(climbing_stairs_dp_array(5))   # 8
