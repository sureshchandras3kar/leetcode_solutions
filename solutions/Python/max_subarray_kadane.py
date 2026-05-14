def maxSubArray(nums):
    """
    Kadane's Algorithm - Find the maximum sum of any contiguous subarray.

    Time Complexity: O(n) - Single pass through the array
    Space Complexity: O(1) - Only tracking current and max sums

    Args:
        nums: List of integers

    Returns:
        Integer representing the maximum subarray sum
    """
    max_sum = nums[0]
    current_sum = nums[0]

    for i in range(1, len(nums)):
        # Decide whether to extend the existing subarray or start fresh
        current_sum = max(nums[i], current_sum + nums[i])
        # Update overall maximum
        max_sum = max(max_sum, current_sum)

    return max_sum
