package main

import "fmt"

func permutationsUsedArray(nums []int) [][]int {
	var result [][]int
	used := make([]bool, len(nums))
	var current []int

	var backtrack func()
	backtrack = func() {
		// Base case: we've used all numbers
		if len(current) == len(nums) {
			perm := make([]int, len(current))
			copy(perm, current)
			result = append(result, perm)
			return
		}

		for i := 0; i < len(nums); i++ {
			if used[i] {
				continue
			}

			// Choose
			current = append(current, nums[i])
			used[i] = true
			// Explore
			backtrack()
			// Unchoose
			current = current[:len(current)-1]
			used[i] = false
		}
	}

	backtrack()
	return result
}

func main() {
	fmt.Println(permutationsUsedArray([]int{1, 2, 3}))
	// Output: [[1 2 3] [1 3 2] [2 1 3] [2 3 1] [3 1 2] [3 2 1]]
}
