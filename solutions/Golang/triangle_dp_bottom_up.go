package main

import (
	"fmt"
)

func minimumTotalBottomUp(triangle [][]int) int {
	/*
	DP Bottom-Up approach for Triangle problem.
	Builds the solution from the bottom row upward.
	Time: O(n²), Space: O(n)
	*/
	n := len(triangle)
	dp := make([]int, len(triangle[n-1]))
	copy(dp, triangle[n-1])

	// Work backwards from the second-to-last row to the top
	for i := n - 2; i >= 0; i-- {
		for j := 0; j < len(triangle[i]); j++ {
			if dp[j] > dp[j+1] {
				dp[j] = triangle[i][j] + dp[j+1]
			} else {
				dp[j] = triangle[i][j] + dp[j]
			}
		}
	}

	return dp[0]
}

func main() {
	triangle := [][]int{
		{2},
		{3, 4},
		{6, 5, 7},
		{4, 1, 8, 3},
	}
	fmt.Println(minimumTotalBottomUp(triangle))  // 11
}
