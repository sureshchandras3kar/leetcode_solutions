package main

import "fmt"

func permutationsSwap(nums []int) [][]int {
	var result [][]int
	backtrack(nums, 0, &result)
	return result
}

func backtrack(nums []int, first int, result *[][]int) {
	// Base case: all elements are placed
	if first == len(nums) {
		perm := make([]int, len(nums))
		copy(perm, nums)
		*result = append(*result, perm)
		return
	}

	for i := first; i < len(nums); i++ {
		// Swap
		nums[first], nums[i] = nums[i], nums[first]
		// Backtrack
		backtrack(nums, first+1, result)
		// Swap back
		nums[first], nums[i] = nums[i], nums[first]
	}
}

func main() {
	fmt.Println(permutationsSwap([]int{1, 2, 3}))
	// Output: [[1 2 3] [1 3 2] [2 1 3] [2 3 1] [3 1 2] [3 2 1]]
}
