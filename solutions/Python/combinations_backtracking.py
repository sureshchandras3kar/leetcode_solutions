from typing import List


def combinations_backtracking(n: int, k: int) -> List[List[int]]:
    """
    Generate all combinations of k numbers from 1 to n using backtracking.
    Time: O(C(n,k) * k), Space: O(C(n,k) * k) for result
    """
    result = []

    def backtrack(start: int, current: List[int]) -> None:
        # Base case: we've selected k numbers
        if len(current) == k:
            result.append(current[:])  # Make a copy
            return

        # Explore: try each remaining number
        for i in range(start, n + 1):
            # Choose
            current.append(i)
            # Explore
            backtrack(i + 1, current)
            # Unchoose
            current.pop()

    backtrack(1, [])
    return result


print(combinations_backtracking(4, 2))  # [[1,2], [1,3], [1,4], [2,3], [2,4], [3,4]]
print(combinations_backtracking(1, 1))  # [[1]]
print(combinations_backtracking(5, 3))  # 10 combinations
