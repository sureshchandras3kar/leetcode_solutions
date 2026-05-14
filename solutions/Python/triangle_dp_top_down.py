from typing import List


def minimumTotal_top_down(triangle: List[List[int]]) -> int:
    """
    DP Top-Down (Memoization) approach for Triangle problem.
    Starts from the top and explores paths downward.
    Time: O(n²), Space: O(n²) for memoization
    """
    memo = {}

    def dfs(row: int, col: int) -> int:
        # Base case: at the bottom
        if row == len(triangle) - 1:
            return triangle[row][col]

        # Check memo
        if (row, col) in memo:
            return memo[(row, col)]

        # Current value + minimum of two possible next positions
        result = triangle[row][col] + min(
            dfs(row + 1, col),
            dfs(row + 1, col + 1)
        )
        memo[(row, col)] = result
        return result

    return dfs(0, 0)


print(minimumTotal_top_down([[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]]))  # 11
