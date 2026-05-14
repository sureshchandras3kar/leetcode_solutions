package main

import "fmt"

/*
 * Find length of longest increasing subsequence using DP O(n²).
 *
 * Time Complexity: O(n²)
 * Space Complexity: O(n)
 */
func lisDP(nums []int) int {
	if len(nums) == 0 {
		return 0
	}

	dp := make([]int, len(nums))
	for i := 0; i < len(nums); i++ {
		dp[i] = 1
	}

	for i := 1; i < len(nums); i++ {
		for j := 0; j < i; j++ {
			if nums[j] < nums[i] {
				if dp[i] < dp[j]+1 {
					dp[i] = dp[j] + 1
				}
			}
		}
	}

	maxVal := 0
	for _, val := range dp {
		if val > maxVal {
			maxVal = val
		}
	}
	return maxVal
}

func main() {
	fmt.Println(lisDP([]int{10, 9, 2, 5, 3, 7, 101, 18}))  // 4
}
