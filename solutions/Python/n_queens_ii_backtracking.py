def totalNQueens(n: int) -> int:
    """Backtracking approach: O(N!) time, O(N^2) space"""
    def is_safe(board, row, col):
        for i in range(row):
            if board[i] == col or abs(board[i] - col) == abs(i - row):
                return False
        return True

    def backtrack(board, row):
        if row == n:
            return 1
        count = 0
        for col in range(n):
            if is_safe(board, row, col):
                board[row] = col
                count += backtrack(board, row + 1)
                board[row] = -1
        return count

    return backtrack([-1] * n, 0)
