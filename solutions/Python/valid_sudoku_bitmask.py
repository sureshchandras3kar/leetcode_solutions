from typing import List


def isValidSudoku(board: List[List[str]]) -> bool:
    rows = [0] * 9
    cols = [0] * 9
    boxes = [0] * 9

    for r in range(9):
        for c in range(9):
            d = board[r][c]
            if d == '.':
                continue
            bit = 1 << (int(d) - 1)
            box = (r // 3) * 3 + (c // 3)
            if rows[r] & bit or cols[c] & bit or boxes[box] & bit:
                return False
            rows[r] |= bit
            cols[c] |= bit
            boxes[box] |= bit

    return True
