def maxSubArray(nums):
    """
    Dynamic Programming approach - Build up subarray sums.

    Time Complexity: O(n)
    Space Complexity: O(1) - Modified from O(n) if we only track current/previous

    Args:
        nums: List of integers

    Returns:
        Integer representing the maximum subarray sum
    """
    n = len(nums)
    if n == 1:
        return nums[0]

    # dp[i] = max sum of subarray ending at index i
    max_sum = nums[0]
    dp = nums[0]

    for i in range(1, n):
        # Either extend the previous subarray or start a new one
        dp = max(nums[i], dp + nums[i])
        max_sum = max(max_sum, dp)

    return max_sum
