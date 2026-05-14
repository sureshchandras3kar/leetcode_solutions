from typing import List


def permutations_swap(nums: List[int]) -> List[List[int]]:
    """
    Generate all permutations using backtracking with swapping approach.
    Time: O(n! * n), Space: O(n!) for result
    """
    result = []

    def backtrack(first: int) -> None:
        # Base case: all elements are placed
        if first == len(nums):
            result.append(nums[:])  # Make a copy
            return

        for i in range(first, len(nums)):
            # Swap
            nums[first], nums[i] = nums[i], nums[first]
            # Backtrack
            backtrack(first + 1)
            # Swap back
            nums[first], nums[i] = nums[i], nums[first]

    backtrack(0)
    return result


print(permutations_swap([1, 2, 3]))
# Output: [[1,2,3], [1,3,2], [2,1,3], [2,3,1], [3,1,2], [3,2,1]]
