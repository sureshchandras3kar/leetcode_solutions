package main

/**
 * Dynamic Programming approach - Build up subarray sums.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1) - Optimized space version
 */
func maxSubArray(nums []int) int {
    n := len(nums)
    if n == 1 {
        return nums[0]
    }

    maxSum := nums[0]
    dp := nums[0]  // max sum ending at current position

    for i := 1; i < n; i++ {
        if nums[i] > dp+nums[i] {
            dp = nums[i]
        } else {
            dp = dp + nums[i]
        }
        if dp > maxSum {
            maxSum = dp
        }
    }

    return maxSum
}
