from typing import List


def permutations_used_array(nums: List[int]) -> List[List[int]]:
    """
    Generate all permutations using backtracking with used array.
    Time: O(n! * n), Space: O(n!) for result
    """
    result = []
    used = [False] * len(nums)

    def backtrack(current: List[int]) -> None:
        # Base case: we've used all numbers
        if len(current) == len(nums):
            result.append(current[:])  # Make a copy
            return

        for i in range(len(nums)):
            if used[i]:
                continue

            # Choose
            current.append(nums[i])
            used[i] = True
            # Explore
            backtrack(current)
            # Unchoose
            current.pop()
            used[i] = False

    backtrack([])
    return result


print(permutations_used_array([1, 2, 3]))
# Output: [[1,2,3], [1,3,2], [2,1,3], [2,3,1], [3,1,2], [3,2,1]]
