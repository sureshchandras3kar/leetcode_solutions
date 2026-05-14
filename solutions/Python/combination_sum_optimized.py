from typing import List


def combination_sum_optimized(candidates: List[int], target: int) -> List[List[int]]:
    """
    Find all unique combinations that sum to target using optimized backtracking.
    Optimization: skip candidates that are too large.
    Time: O(N^T), Space: O(T) for recursion depth
    """
    candidates.sort()  # Sort to enable pruning
    result = []

    def backtrack(index: int, current: List[int], remaining: int) -> None:
        # Base case: found a valid combination
        if remaining == 0:
            result.append(current[:])
            return

        # Explore: try each candidate from index onwards
        for i in range(index, len(candidates)):
            candidate = candidates[i]

            # Pruning: if candidate > remaining, all further candidates will be too large
            if candidate > remaining:
                break

            # Choose
            current.append(candidate)
            # Explore: can reuse the same candidate (i, not i+1)
            backtrack(i, current, remaining - candidate)
            # Unchoose
            current.pop()

    backtrack(0, [], target)
    return result


print(combination_sum_optimized([2, 3, 6, 7], 7))
# Output: [[2,2,3], [7]]
