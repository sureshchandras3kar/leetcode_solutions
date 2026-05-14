package main

/*
#35 Search Insert Position - Linear Search Approach
Time: O(n), Space: O(1)

Iterate through array to find position or insertion point.
*/

func SearchInsertLinearSearch(nums []int, target int) int {
	/*
	Linear search through array.
	Return index when found or where it should be inserted.
	*/
	for i := 0; i < len(nums); i++ {
		if nums[i] >= target {
			return i
		}
	}

	// If target larger than all elements
	return len(nums)
}
