from typing import List


def rotate_array_reverse(nums: List[int], k: int) -> None:
    """
    Rotate array in-place using reverse trick.

    Time: O(n) | Space: O(1)

    Approach: Reverse the entire array, then reverse first k elements,
    then reverse remaining n-k elements. This achieves rotation without
    extra space.
    """
    if not nums or k == 0:
        return

    k = k % len(nums)  # Handle k > n
    if k == 0:
        return

    def reverse(arr: List[int], start: int, end: int) -> None:
        while start < end:
            arr[start], arr[end] = arr[end], arr[start]
            start += 1
            end -= 1

    # Reverse entire array: [1,2,3,4,5] -> [5,4,3,2,1]
    reverse(nums, 0, len(nums) - 1)
    # Reverse first k: [5,4,3,2,1] -> [3,4,5,2,1]
    reverse(nums, 0, k - 1)
    # Reverse rest: [3,4,5,2,1] -> [3,4,5,1,2]
    reverse(nums, k, len(nums) - 1)


# Test with expected output
nums = [1, 2, 3, 4, 5]
rotate_array_reverse(nums, 2)
print(nums)  # [4, 5, 1, 2, 3]
