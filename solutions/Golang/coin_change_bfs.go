package main

import (
	"fmt"
)

/*
 * Find minimum number of coins needed to make amount using BFS.
 *
 * Time Complexity: O(n * amount)
 * Space Complexity: O(amount)
 */
func coinChangeBFS(coins []int, amount int) int {
	if amount == 0 {
		return 0
	}

	type state struct {
		amount   int
		numCoins int
	}

	queue := []state{{amount, 0}}
	visited := make(map[int]bool)
	visited[amount] = true

	for len(queue) > 0 {
		curr := queue[0]
		queue = queue[1:]

		for _, coin := range coins {
			nextAmount := curr.amount - coin
			if nextAmount == 0 {
				return curr.numCoins + 1
			}
			if nextAmount > 0 && !visited[nextAmount] {
				visited[nextAmount] = true
				queue = append(queue, state{nextAmount, curr.numCoins + 1})
			}
		}
	}

	return -1
}

func main() {
	fmt.Println(coinChangeBFS([]int{1, 2, 5}, 5))   // 1
	fmt.Println(coinChangeBFS([]int{2}, 3))        // -1
}
