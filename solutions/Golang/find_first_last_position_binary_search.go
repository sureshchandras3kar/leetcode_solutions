package main

/*
#34 Find First and Last Position of Element in Sorted Array - Binary Search
Time: O(log n), Space: O(1)

Find the starting and ending position of a given target value in a sorted array.
Use two binary searches: one to find first position, one to find last.
*/

func SearchRangeBinarySearch(nums []int, target int) []int {
	/*
	Two binary searches to find first and last positions.
	*/
	if len(nums) == 0 {
		return []int{-1, -1}
	}

	// Find first position
	left, right := 0, len(nums)-1
	firstPos := -1

	for left <= right {
		mid := left + (right-left)/2
		if nums[mid] == target {
			firstPos = mid
			right = mid - 1  // Continue searching left
		} else if nums[mid] < target {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}

	// If target not found
	if firstPos == -1 {
		return []int{-1, -1}
	}

	// Find last position
	left = 0
	right = len(nums) - 1
	lastPos := -1

	for left <= right {
		mid := left + (right-left)/2
		if nums[mid] == target {
			lastPos = mid
			left = mid + 1  // Continue searching right
		} else if nums[mid] < target {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}

	return []int{firstPos, lastPos}
}
