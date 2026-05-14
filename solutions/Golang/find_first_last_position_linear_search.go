package main

/*
#34 Find First and Last Position of Element in Sorted Array - Linear Search
Time: O(n), Space: O(1)

Linear scan to find first and last occurrences of target value.
*/

func SearchRangeLinearSearch(nums []int, target int) []int {
	/*
	Linear search to find first and last positions.
	*/
	if len(nums) == 0 {
		return []int{-1, -1}
	}

	firstPos := -1
	lastPos := -1

	// Find first position
	for i := 0; i < len(nums); i++ {
		if nums[i] == target {
			firstPos = i
			break
		}
	}

	// If target not found
	if firstPos == -1 {
		return []int{-1, -1}
	}

	// Find last position
	for i := len(nums) - 1; i >= 0; i-- {
		if nums[i] == target {
			lastPos = i
			break
		}
	}

	return []int{firstPos, lastPos}
}
