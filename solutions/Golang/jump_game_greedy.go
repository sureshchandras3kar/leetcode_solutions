package main

import "fmt"

// Greedy approach: work backwards from the end to find the furthest index
// we can reach. If we can reach index 0, we can reach the end.
//
// Time: O(n), Space: O(1)
func canJumpGreedy(nums []int) bool {
	lastGoodIndex := len(nums) - 1
	for i := len(nums) - 2; i >= 0; i-- {
		if i+nums[i] >= lastGoodIndex {
			lastGoodIndex = i
		}
	}
	return lastGoodIndex == 0
}

func main() {
	fmt.Println(canJumpGreedy([]int{2, 3, 1, 1, 4}))  // true
	fmt.Println(canJumpGreedy([]int{3, 2, 1, 0, 4}))  // false
	fmt.Println(canJumpGreedy([]int{0}))              // true
	fmt.Println(canJumpGreedy([]int{2, 0, 0}))        // true
	fmt.Println(canJumpGreedy([]int{0, 2, 3}))        // false
}
