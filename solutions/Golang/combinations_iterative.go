package main

import "fmt"

func combinationsIterative(n int, k int) [][]int {
	var result [][]int
	combo := make([]int, k)

	// Initialize with [1, 2, ..., k]
	for i := 0; i < k; i++ {
		combo[i] = i + 1
	}

	for {
		// Save current combination
		current := make([]int, k)
		copy(current, combo)
		result = append(result, current)

		// Find the rightmost number that can be incremented
		i := k - 1
		for i >= 0 && combo[i] == n-k+i+1 {
			i--
		}

		if i < 0 {
			break
		}

		// Increment and reset
		combo[i]++
		for j := i + 1; j < k; j++ {
			combo[j] = combo[j-1] + 1
		}
	}

	return result
}

func main() {
	fmt.Println(combinationsIterative(4, 2))
	// Output: [[1 2] [1 3] [1 4] [2 3] [2 4] [3 4]]
}
