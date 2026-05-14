package main

import "fmt"

/*
 * Climb n stairs with dp array approach.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */
func climbingStairsDpArray(n int) int {
	if n <= 1 {
		return 1
	}

	dp := make([]int, n+1)
	dp[0] = 1
	dp[1] = 1

	for i := 2; i <= n; i++ {
		dp[i] = dp[i-1] + dp[i-2]
	}

	return dp[n]
}

func main() {
	fmt.Println(climbingStairsDpArray(3))  // 3
	fmt.Println(climbingStairsDpArray(4))  // 5
	fmt.Println(climbingStairsDpArray(5))  // 8
}
