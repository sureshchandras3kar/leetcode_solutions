func productExceptSelf(nums []int) []int {
	n := len(nums)
	output := make([]int, n)

	prefix := 1
	for i := 0; i < n; i++ {
		output[i] = prefix
		prefix *= nums[i]
	}

	suffix := 1
	for i := n - 1; i >= 0; i-- {
		output[i] *= suffix
		suffix *= nums[i]
	}

	return output
}
