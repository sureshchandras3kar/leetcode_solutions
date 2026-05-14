package main

import "fmt"

/*
 * Climb n stairs with Fibonacci variables approach (space-optimized).
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 */
func climbingStairsFibonacciVariables(n int) int {
	if n <= 1 {
		return 1
	}

	prev2 := 1  // dp[0]
	prev1 := 1  // dp[1]

	for i := 2; i <= n; i++ {
		current := prev1 + prev2
		prev2 = prev1
		prev1 = current
	}

	return prev1
}

func main() {
	fmt.Println(climbingStairsFibonacciVariables(3))  // 3
	fmt.Println(climbingStairsFibonacciVariables(4))  // 5
	fmt.Println(climbingStairsFibonacciVariables(5))  // 8
}
