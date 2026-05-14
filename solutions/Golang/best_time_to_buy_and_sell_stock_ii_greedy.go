package main

import "fmt"

func bestTimeToBuyAndSellStockIIGreedy(prices []int) int {
	maxProfit := 0
	for i := 1; i < len(prices); i++ {
		if prices[i] > prices[i-1] {
			maxProfit += prices[i] - prices[i-1]
		}
	}
	return maxProfit
}

func main() {
	prices1 := []int{7, 1, 5, 3, 6, 4}
	fmt.Println(bestTimeToBuyAndSellStockIIGreedy(prices1))  // 7

	prices2 := []int{1, 2, 3, 4, 5}
	fmt.Println(bestTimeToBuyAndSellStockIIGreedy(prices2))  // 4

	prices3 := []int{7, 6, 4, 3, 1}
	fmt.Println(bestTimeToBuyAndSellStockIIGreedy(prices3))  // 0
}
