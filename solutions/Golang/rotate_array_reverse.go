package main

import "fmt"

func rotateArrayReverse(nums []int, k int) {
	/**
	 * Rotate array in-place using reverse trick.
	 *
	 * Time: O(n) | Space: O(1)
	 *
	 * Approach: Reverse the entire array, then reverse first k elements,
	 * then reverse remaining n-k elements. This achieves rotation without
	 * extra space.
	 */
	if len(nums) == 0 || k == 0 {
		return
	}

	n := len(nums)
	k = k % n  // Handle k > n
	if k == 0 {
		return
	}

	reverse := func(start, end int) {
		for start < end {
			nums[start], nums[end] = nums[end], nums[start]
			start++
			end--
		}
	}

	// Reverse entire array: [1,2,3,4,5] -> [5,4,3,2,1]
	reverse(0, n-1)
	// Reverse first k: [5,4,3,2,1] -> [3,4,5,2,1]
	reverse(0, k-1)
	// Reverse rest: [3,4,5,2,1] -> [3,4,5,1,2]
	reverse(k, n-1)
}

func main() {
	nums := []int{1, 2, 3, 4, 5}
	rotateArrayReverse(nums, 2)
	fmt.Println(nums)  // [4, 5, 1, 2, 3]
}
