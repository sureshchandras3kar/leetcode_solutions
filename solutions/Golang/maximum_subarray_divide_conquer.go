package main

import (
	"fmt"
	"math"
)

/**
 * Divide and Conquer approach: Split the array, find max subarrays in each half,
 * and at the crossing point, then return the maximum.
 * Time: O(n log n), Space: O(log n) for recursion stack
 */
func maximumSubarrayDivideConquer(nums []int) int {
	return helper(nums, 0, len(nums)-1)
}

func helper(nums []int, left int, right int) int {
	if left == right {
		return nums[left]
	}

	mid := left + (right-left)/2

	// Maximum in left half, right half, and crossing the middle
	left_max := helper(nums, left, mid)
	right_max := helper(nums, mid+1, right)
	cross_max := maxCrossingSubarray(nums, left, mid, right)

	return max(left_max, max(right_max, cross_max))
}

func maxCrossingSubarray(nums []int, left int, mid int, right int) int {
	// Sum from mid to left
	left_sum := math.MinInt
	sum := 0
	for i := mid; i >= left; i-- {
		sum += nums[i]
		if sum > left_sum {
			left_sum = sum
		}
	}

	// Sum from mid+1 to right
	right_sum := math.MinInt
	sum = 0
	for i := mid + 1; i <= right; i++ {
		sum += nums[i]
		if sum > right_sum {
			right_sum = sum
		}
	}

	return left_sum + right_sum
}

func max(a int, b int) int {
	if a > b {
		return a
	}
	return b
}

func main() {
	fmt.Println(maximumSubarrayDivideConquer([]int{-2, 1, -3, 4, -1, 2, 1, -5, 4}))  // 6
	fmt.Println(maximumSubarrayDivideConquer([]int{5}))  // 5
	fmt.Println(maximumSubarrayDivideConquer([]int{-5, -2, -3}))  // -2
}
