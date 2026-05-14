from typing import List

def exist(board: List[List[str]], word: str) -> bool:
    """Backtracking approach: O(N*M*4^L) time, O(L) space"""
    if not board:
        return False

    def backtrack(row, col, index):
        if index == len(word):
            return True
        if row < 0 or row >= len(board) or col < 0 or col >= len(board[0]):
            return False
        if board[row][col] != word[index]:
            return False

        board[row][col] = "#"
        found = (backtrack(row + 1, col, index + 1) or
                 backtrack(row - 1, col, index + 1) or
                 backtrack(row, col + 1, index + 1) or
                 backtrack(row, col - 1, index + 1))
        board[row][col] = word[index]
        return found

    for i in range(len(board)):
        for j in range(len(board[0])):
            if backtrack(i, j, 0):
                return True
    return False
