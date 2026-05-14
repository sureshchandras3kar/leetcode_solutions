package main

/**
 * Kadane's Algorithm - Find the maximum sum of any contiguous subarray.
 *
 * Time Complexity: O(n) - Single pass through the array
 * Space Complexity: O(1) - Only tracking current and max sums
 */
func maxSubArray(nums []int) int {
    maxSum := nums[0]
    currentSum := nums[0]

    for i := 1; i < len(nums); i++ {
        // Decide whether to extend the existing subarray or start fresh
        if currentSum+nums[i] < nums[i] {
            currentSum = nums[i]
        } else {
            currentSum = currentSum + nums[i]
        }
        // Update overall maximum
        if currentSum > maxSum {
            maxSum = currentSum
        }
    }

    return maxSum
}
