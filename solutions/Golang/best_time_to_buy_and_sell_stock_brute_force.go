// Approach: Brute Force
// Try every pair (i, j) where i < j. Profit = prices[j] - prices[i].
// Track the maximum profit across all pairs.
//
// Time Complexity: O(n²) — nested loops over all pairs
// Space Complexity: O(1)

package main

func maxProfitBruteForce(prices []int) int {
	n := len(prices)
	maxProfitVal := 0

	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			profit := prices[j] - prices[i]
			if profit > maxProfitVal {
				maxProfitVal = profit
			}
		}
	}

	return maxProfitVal
}

// Test cases:
// maxProfitBruteForce([]int{7, 1, 5, 3, 6, 4})  // 5
// maxProfitBruteForce([]int{7, 6, 4, 3, 1})     // 0
// maxProfitBruteForce([]int{2, 4, 1, 7, 5, 11}) // 9
