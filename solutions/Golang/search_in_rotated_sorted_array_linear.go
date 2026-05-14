package main

/*
#33 Search in Rotated Sorted Array - Linear Search Approach
Time: O(n), Space: O(1)

Linear scan through array to find target value.
*/

func SearchInRotatedSortedArrayLinear(nums []int, target int) int {
	/*
	Simple linear search through array.
	Return index if found, otherwise -1.
	*/
	for i := 0; i < len(nums); i++ {
		if nums[i] == target {
			return i
		}
	}

	return -1
}
