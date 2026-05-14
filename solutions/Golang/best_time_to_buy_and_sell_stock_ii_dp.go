package main

import "fmt"

func bestTimeToBuyAndSellStockIIDP(prices []int) int {
	if len(prices) < 2 {
		return 0
	}

	hold := -prices[0]  // profit if holding stock after day 0
	noHold := 0         // profit if not holding stock after day 0

	for i := 1; i < len(prices); i++ {
		// Either hold from before or buy today (reset from noHold)
		if noHold-prices[i] > hold {
			hold = noHold - prices[i]
		}
		// Either don't hold from before or sell today (from hold)
		if hold+prices[i] > noHold {
			noHold = hold + prices[i]
		}
	}

	return noHold
}

func main() {
	prices1 := []int{7, 1, 5, 3, 6, 4}
	fmt.Println(bestTimeToBuyAndSellStockIIDP(prices1))  // 7

	prices2 := []int{1, 2, 3, 4, 5}
	fmt.Println(bestTimeToBuyAndSellStockIIDP(prices2))  // 4

	prices3 := []int{7, 6, 4, 3, 1}
	fmt.Println(bestTimeToBuyAndSellStockIIDP(prices3))  // 0
}
