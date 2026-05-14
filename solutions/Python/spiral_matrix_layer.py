from typing import List


def spiral_matrix_layer(matrix: List[List[int]]) -> List[int]:
    """
    Traverse the matrix in spiral order by peeling off layers.

    Time Complexity: O(m * n) where m is rows and n is columns
    Space Complexity: O(1) excluding the result list
    """
    if not matrix or not matrix[0]:
        return []

    result = []
    m, n = len(matrix), len(matrix[0])
    layers = (min(m, n) + 1) // 2

    for layer in range(layers):
        top = layer
        bottom = m - 1 - layer
        left = layer
        right = n - 1 - layer

        # Traverse right
        for col in range(left, right + 1):
            result.append(matrix[top][col])

        # Traverse down
        for row in range(top + 1, bottom + 1):
            result.append(matrix[row][right])

        # Traverse left (if there's a row remaining)
        if top < bottom:
            for col in range(right - 1, left - 1, -1):
                result.append(matrix[bottom][col])

        # Traverse up (if there's a column remaining)
        if left < right:
            for row in range(bottom - 1, top, -1):
                result.append(matrix[row][left])

    return result


# Test cases
print(spiral_matrix_layer([[1, 2, 3], [4, 5, 6], [7, 8, 9]]))  # [1, 2, 3, 6, 9, 8, 7, 4, 5]
print(spiral_matrix_layer([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]))  # [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
