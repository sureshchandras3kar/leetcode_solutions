package main

import "fmt"

func rotateArrayExtraSpace(nums []int, k int) {
	/**
	 * Rotate array using extra space.
	 *
	 * Time: O(n) | Space: O(n)
	 *
	 * Approach: Create a new array where element at index i goes to index
	 * (i + k) % n. Copy back to original array.
	 */
	if len(nums) == 0 || k == 0 {
		return
	}

	n := len(nums)
	k = k % n  // Handle k > n
	if k == 0 {
		return
	}

	// Create rotated result
	rotated := make([]int, n)
	for i := 0; i < n; i++ {
		rotated[(i+k)%n] = nums[i]
	}

	// Copy back to original array
	for i := 0; i < n; i++ {
		nums[i] = rotated[i]
	}
}

func main() {
	nums := []int{1, 2, 3, 4, 5}
	rotateArrayExtraSpace(nums, 2)
	fmt.Println(nums)  // [4, 5, 1, 2, 3]
}
