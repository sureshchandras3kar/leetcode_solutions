// Approach: Single Pass with Min Price Tracking
// Track the minimum price seen so far and calculate the maximum profit at each price.
// When we see a new price, we check the profit if we sell at that price.
//
// Time Complexity: O(n) — single pass
// Space Complexity: O(1)

package main

func maxProfit(prices []int) int {
	if len(prices) < 2 {
		return 0
	}

	minPrice := prices[0]
	maxProfitVal := 0

	for i := 1; i < len(prices); i++ {
		profit := prices[i] - minPrice
		if profit > maxProfitVal {
			maxProfitVal = profit
		}
		if prices[i] < minPrice {
			minPrice = prices[i]
		}
	}

	return maxProfitVal
}

// Test cases:
// maxProfit([]int{7, 1, 5, 3, 6, 4})  // 5
// maxProfit([]int{7, 6, 4, 3, 1})     // 0
// maxProfit([]int{2, 4, 1, 7, 5, 11}) // 9
