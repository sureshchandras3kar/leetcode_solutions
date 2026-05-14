package main

/*
#35 Search Insert Position - Binary Search Approach
Time: O(log n), Space: O(1)

Given a sorted array and a target value, return the index if found,
otherwise return the index where it would be if it were inserted.
*/

func SearchInsertBinarySearch(nums []int, target int) int {
	/*
	Binary search to find insert position.
	Left pointer converges to insertion point.
	*/
	left, right := 0, len(nums)-1

	for left <= right {
		mid := left + (right-left)/2

		if nums[mid] == target {
			return mid
		} else if nums[mid] < target {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}

	// left is at insertion position when target not found
	return left
}
