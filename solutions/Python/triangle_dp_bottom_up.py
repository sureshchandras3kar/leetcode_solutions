from typing import List


def minimumTotal_bottom_up(triangle: List[List[int]]) -> int:
    """
    DP Bottom-Up approach for Triangle problem.
    Builds the solution from the bottom row upward.
    Time: O(n²), Space: O(n) where n is the number of rows
    """
    n = len(triangle)
    dp = triangle[-1][:]  # Copy last row

    # Work backwards from the second-to-last row to the top
    for i in range(n - 2, -1, -1):
        for j in range(len(triangle[i])):
            # Current position can come from two paths in the row below
            dp[j] = triangle[i][j] + min(dp[j], dp[j + 1])

    return dp[0]


print(minimumTotal_bottom_up([[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]]))  # 11
