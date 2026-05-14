package main

import (
	"fmt"
	"math"
)

/*
 * Find minimum number of coins needed to make amount using DP array.
 *
 * Time Complexity: O(n * amount)
 * Space Complexity: O(amount)
 */
func coinChangeDP(coins []int, amount int) int {
	dp := make([]int, amount+1)
	for i := 1; i <= amount; i++ {
		dp[i] = math.MaxInt32
	}
	dp[0] = 0

	for i := 1; i <= amount; i++ {
		for _, coin := range coins {
			if coin <= i && dp[i-coin] != math.MaxInt32 {
				if dp[i-coin]+1 < dp[i] {
					dp[i] = dp[i-coin] + 1
				}
			}
		}
	}

	if dp[amount] == math.MaxInt32 {
		return -1
	}
	return dp[amount]
}

func main() {
	fmt.Println(coinChangeDP([]int{1, 2, 5}, 5))   // 1
	fmt.Println(coinChangeDP([]int{2}, 3))        // -1
}
