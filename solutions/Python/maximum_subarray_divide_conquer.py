from typing import List, Tuple


def maximum_subarray_divide_conquer(nums: List[int]) -> int:
    """
    Divide and Conquer approach: Split the array, find max subarrays in each half,
    and at the crossing point, then return the maximum.

    Time: O(n log n), Space: O(log n) for recursion stack
    """

    def max_crossing_subarray(nums: List[int], left: int, mid: int, right: int) -> int:
        """Find maximum subarray sum that crosses the middle point."""
        # Sum from mid to left
        left_sum = float("-inf")
        sum_val = 0
        for i in range(mid, left - 1, -1):
            sum_val += nums[i]
            left_sum = max(left_sum, sum_val)

        # Sum from mid+1 to right
        right_sum = float("-inf")
        sum_val = 0
        for i in range(mid + 1, right + 1):
            sum_val += nums[i]
            right_sum = max(right_sum, sum_val)

        return left_sum + right_sum

    def helper(nums: List[int], left: int, right: int) -> int:
        """Recursively find the maximum subarray sum."""
        if left == right:
            return nums[left]

        mid = (left + right) // 2

        # Maximum in left half, right half, and crossing the middle
        left_max = helper(nums, left, mid)
        right_max = helper(nums, mid + 1, right)
        cross_max = max_crossing_subarray(nums, left, mid, right)

        return max(left_max, right_max, cross_max)

    return helper(nums, 0, len(nums) - 1)


print(maximum_subarray_divide_conquer([-2, 1, -3, 4, -1, 2, 1, -5, 4]))  # 6
print(maximum_subarray_divide_conquer([5]))  # 5
print(maximum_subarray_divide_conquer([-5, -2, -3]))  # -2
