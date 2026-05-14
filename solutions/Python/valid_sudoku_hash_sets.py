from typing import List
from collections import defaultdict


def isValidSudoku(board: List[List[str]]) -> bool:
    rows = defaultdict(set)
    cols = defaultdict(set)
    boxes = defaultdict(set)

    for r in range(9):
        for c in range(9):
            d = board[r][c]
            if d == '.':
                continue
            box = (r // 3) * 3 + (c // 3)
            if d in rows[r] or d in cols[c] or d in boxes[box]:
                return False
            rows[r].add(d)
            cols[c].add(d)
            boxes[box].add(d)

    return True
