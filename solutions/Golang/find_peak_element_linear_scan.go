package main

/*
#162 Find Peak Element - Linear Scan Approach
Time: O(n), Space: O(1)

Iterate through array and find the first element greater than its neighbors.
*/

func FindPeakElementLinearScan(nums []int) int {
	/*
	Linear scan to find first peak.
	Compare each element with its neighbors.
	*/
	for i := 0; i < len(nums); i++ {
		// Check if current element is greater than neighbors
		isPeak := true

		if i > 0 && nums[i] <= nums[i-1] {
			isPeak = false
		}

		if i < len(nums)-1 && nums[i] <= nums[i+1] {
			isPeak = false
		}

		if isPeak {
			return i
		}
	}

	// If no peak found (shouldn't happen given constraints)
	return 0
}
