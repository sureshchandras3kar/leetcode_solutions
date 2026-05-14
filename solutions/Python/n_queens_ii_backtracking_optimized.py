def totalNQueens(n: int) -> int:
    """Optimized backtracking: O(N!) time, O(N) space"""
    def backtrack(row, cols, diag1, diag2):
        if row == n:
            return 1
        count = 0
        for col in range(n):
            if col not in cols and (row - col) not in diag1 and (row + col) not in diag2:
                count += backtrack(row + 1, cols | {col}, diag1 | {row - col}, diag2 | {row + col})
        return count

    return backtrack(0, set(), set(), set())
