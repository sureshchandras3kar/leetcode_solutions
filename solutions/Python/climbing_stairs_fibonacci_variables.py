def climbing_stairs_fibonacci_variables(n: int) -> int:
    """
    Climb n stairs with Fibonacci variables approach (space-optimized).

    Time Complexity: O(n)
    Space Complexity: O(1)

    Instead of storing all values in an array, we only keep the last two
    values since dp[i] only depends on dp[i-1] and dp[i-2].
    """
    if n <= 1:
        return 1

    prev2 = 1  # dp[0]
    prev1 = 1  # dp[1]

    for i in range(2, n + 1):
        current = prev1 + prev2
        prev2 = prev1
        prev1 = current

    return prev1


if __name__ == "__main__":
    print(climbing_stairs_fibonacci_variables(3))   # 3 (1+1+1, 1+2, 2+1)
    print(climbing_stairs_fibonacci_variables(4))   # 5
    print(climbing_stairs_fibonacci_variables(5))   # 8
