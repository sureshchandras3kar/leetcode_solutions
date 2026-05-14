package main

import (
	"fmt"
	"sort"
)

func combinationSumOptimized(candidates []int, target int) [][]int {
	sort.Ints(candidates)  // Sort to enable pruning
	var result [][]int
	var current []int

	var backtrack func(int, []int, int)
	backtrack = func(index int, current []int, remaining int) {
		// Base case: found a valid combination
		if remaining == 0 {
			combo := make([]int, len(current))
			copy(combo, current)
			result = append(result, combo)
			return
		}

		// Explore: try each candidate from index onwards
		for i := index; i < len(candidates); i++ {
			candidate := candidates[i]

			// Pruning: if candidate > remaining, all further candidates will be too large
			if candidate > remaining {
				break
			}

			// Choose
			current = append(current, candidate)
			// Explore: can reuse the same candidate (i, not i+1)
			backtrack(i, current, remaining-candidate)
			// Unchoose
			current = current[:len(current)-1]
		}
	}

	backtrack(0, current, target)
	return result
}

func main() {
	fmt.Println(combinationSumOptimized([]int{2, 3, 6, 7}, 7))
	// Output: [[2 2 3] [7]]
}
