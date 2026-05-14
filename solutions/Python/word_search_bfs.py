from typing import List
from collections import deque

def exist(board: List[List[str]], word: str) -> bool:
    """BFS approach: O(N*M*4^L) time, O(N*M) space"""
    if not board:
        return False

    for i in range(len(board)):
        for j in range(len(board[0])):
            if bfs(board, word, i, j):
                return True
    return False

def bfs(board, word, start_row, start_col):
    queue = deque([(start_row, start_col, 0, set())])
    while queue:
        row, col, index, visited = queue.popleft()
        if index == len(word):
            return True
        for dr, dc in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            nr, nc = row + dr, col + dc
            if 0 <= nr < len(board) and 0 <= nc < len(board[0]):
                if board[nr][nc] == word[index] and (nr, nc) not in visited:
                    visited.add((nr, nc))
                    queue.append((nr, nc, index + 1, visited.copy()))
    return False
