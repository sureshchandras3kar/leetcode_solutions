package main

import "fmt"

func combinationsBacktracking(n int, k int) [][]int {
	var result [][]int
	var current []int

	var backtrack func(int)
	backtrack = func(start int) {
		// Base case: we've selected k numbers
		if len(current) == k {
			combo := make([]int, k)
			copy(combo, current)
			result = append(result, combo)
			return
		}

		// Explore: try each remaining number
		for i := start; i <= n; i++ {
			// Choose
			current = append(current, i)
			// Explore
			backtrack(i + 1)
			// Unchoose
			current = current[:len(current)-1]
		}
	}

	backtrack(1)
	return result
}

func main() {
	fmt.Println(combinationsBacktracking(4, 2))
	// Output: [[1 2] [1 3] [1 4] [2 3] [2 4] [3 4]]
}
