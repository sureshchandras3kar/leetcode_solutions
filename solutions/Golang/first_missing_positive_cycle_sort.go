package golang

func firstMissingPositiveCycleSort(nums []int) int {
	n := len(nums)
	i := 0

	// Place each number at its correct index (value v at index v-1)
	for i < n {
		j := nums[i] - 1
		if nums[i] >= 1 && nums[i] <= n && nums[j] != nums[i] {
			nums[i], nums[j] = nums[j], nums[i]
		} else {
			i++
		}
	}

	// Find first index where value doesn't match expected
	for i := 0; i < n; i++ {
		if nums[i] != i+1 {
			return i + 1
		}
	}

	return n + 1
}
