from typing import List


def rotate_array_extra_space(nums: List[int], k: int) -> None:
    """
    Rotate array using extra space.

    Time: O(n) | Space: O(n)

    Approach: Create a new array where element at index i goes to index
    (i + k) % n. Copy back to original array.
    """
    if not nums or k == 0:
        return

    n = len(nums)
    k = k % n  # Handle k > n
    if k == 0:
        return

    # Create rotated result
    rotated = [0] * n
    for i in range(n):
        rotated[(i + k) % n] = nums[i]

    # Copy back to original array
    for i in range(n):
        nums[i] = rotated[i]


# Test with expected output
nums = [1, 2, 3, 4, 5]
rotate_array_extra_space(nums, 2)
print(nums)  # [4, 5, 1, 2, 3]
