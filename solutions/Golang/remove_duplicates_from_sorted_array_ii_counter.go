package main

import "fmt"

/*
Two Pointers with Counter Approach
Allow each element to appear at most twice using explicit count tracking.

Time Complexity: O(n)
Space Complexity: O(1)
*/
func removeDuplicates(nums []int) int {
	if len(nums) == 0 {
		return 0
	}

	k := 1
	count := 1
	for i := 1; i < len(nums); i++ {
		if nums[i] != nums[i-1] {
			// New element encountered, reset counter
			count = 1
			nums[k] = nums[i]
			k++
		} else if count < 2 {
			// Same element but less than 2 occurrences, allow it
			count++
			nums[k] = nums[i]
			k++
		}
		// else: count == 2, skip this duplicate
	}

	return k
}

func main() {
	nums1 := []int{1, 1, 1, 2, 2, 3}
	fmt.Println(removeDuplicates(nums1))  // 5, nums = [1, 1, 2, 2, 3, _]

	nums2 := []int{0, 0, 1, 1, 1, 1, 2, 3, 3}
	fmt.Println(removeDuplicates(nums2))  // 7, nums = [0, 0, 1, 1, 2, 3, 3, _, _]

	nums3 := []int{1}
	fmt.Println(removeDuplicates(nums3))  // 1, nums = [1]

	nums4 := []int{1, 2}
	fmt.Println(removeDuplicates(nums4))  // 2, nums = [1, 2]
}
