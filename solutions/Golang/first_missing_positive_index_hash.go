package golang

func firstMissingPositiveIndexHash(nums []int) int {
	n := len(nums)

	// Phase 1: replace non-positives and values > n with n+1
	for i := 0; i < n; i++ {
		if nums[i] <= 0 || nums[i] > n {
			nums[i] = n + 1
		}
	}

	// Phase 2: mark presence using negative signs
	for i := 0; i < n; i++ {
		v := nums[i]
		if v < 0 {
			v = -v
		}
		if v >= 1 && v <= n {
			if nums[v-1] > 0 {
				nums[v-1] = -nums[v-1]
			}
		}
	}

	// Phase 3: find first unmarked index
	for i := 0; i < n; i++ {
		if nums[i] > 0 {
			return i + 1
		}
	}

	return n + 1
}
