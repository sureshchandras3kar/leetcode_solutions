package main

import "fmt"

/*
 * Rob houses with maximum money using DP array approach.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */
func houseRobberDpArray(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	if len(nums) == 1 {
		return nums[0]
	}

	dp := make([]int, len(nums))
	dp[0] = nums[0]
	if nums[1] > nums[0] {
		dp[1] = nums[1]
	} else {
		dp[1] = nums[0]
	}

	for i := 2; i < len(nums); i++ {
		rob := nums[i] + dp[i-2]
		skip := dp[i-1]
		if rob > skip {
			dp[i] = rob
		} else {
			dp[i] = skip
		}
	}

	return dp[len(nums)-1]
}

func main() {
	fmt.Println(houseRobberDpArray([]int{1, 2, 3, 1}))      // 4
	fmt.Println(houseRobberDpArray([]int{2, 7, 9, 3, 1}))   // 12
}
