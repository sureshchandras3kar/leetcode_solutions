package main

import "fmt"

// Dynamic programming approach: forward pass tracking the furthest
// reachable index. If we can ever reach the end index, return true.
//
// Time: O(n), Space: O(1)
func canJumpDP(nums []int) bool {
	maxReach := 0
	for i := 0; i < len(nums); i++ {
		if i > maxReach {
			return false
		}
		if i+nums[i] > maxReach {
			maxReach = i + nums[i]
		}
		if maxReach >= len(nums)-1 {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println(canJumpDP([]int{2, 3, 1, 1, 4}))  // true
	fmt.Println(canJumpDP([]int{3, 2, 1, 0, 4}))  // false
	fmt.Println(canJumpDP([]int{0}))              // true
	fmt.Println(canJumpDP([]int{2, 0, 0}))        // true
	fmt.Println(canJumpDP([]int{0, 2, 3}))        // false
}
