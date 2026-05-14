package main

import "fmt"

/*
 * Rob houses with maximum money using space-optimized approach.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 */
func houseRobberSpaceOptimized(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	if len(nums) == 1 {
		return nums[0]
	}

	prev2 := nums[0]
	prev1 := nums[0]
	if nums[1] > nums[0] {
		prev1 = nums[1]
	}

	for i := 2; i < len(nums); i++ {
		rob := nums[i] + prev2
		skip := prev1
		var current int
		if rob > skip {
			current = rob
		} else {
			current = skip
		}
		prev2 = prev1
		prev1 = current
	}

	return prev1
}

func main() {
	fmt.Println(houseRobberSpaceOptimized([]int{1, 2, 3, 1}))      // 4
	fmt.Println(houseRobberSpaceOptimized([]int{2, 7, 9, 3, 1}))   // 12
}
