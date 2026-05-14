from typing import List

def searchMatrix(matrix: List[List[int]], target: int) -> bool:
    """Flatten approach: O(log(m*n)) time, O(1) space"""
    if not matrix or not matrix[0]:
        return False

    row_count = len(matrix)
    col_count = len(matrix[0])

    left, right = 0, row_count - 1

    while left <= right:
        mid_row = (left + right) // 2
        if target < matrix[mid_row][0]:
            right = mid_row - 1
        elif target > matrix[mid_row][-1]:
            left = mid_row + 1
        else:
            return target in matrix[mid_row]

    return False
