package main

import "fmt"

/*
Simple Two Pointers Approach
Allow at most 2 occurrences by checking if current element differs from element 2 positions back.

Time Complexity: O(n)
Space Complexity: O(1)
*/
func removeDuplicates(nums []int) int {
	if len(nums) == 0 {
		return 0
	}

	k := 0
	for i := 0; i < len(nums); i++ {
		// Always write first 2 elements, or if current differs from element 2 positions back
		if k < 2 || nums[i] != nums[k-2] {
			nums[k] = nums[i]
			k++
		}
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
