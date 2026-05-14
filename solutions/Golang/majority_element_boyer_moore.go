package golang
func majorityElement(nums []int) int {
	candidate, count := nums[0], 1
	for _, num := range nums[1:] {
		if count == 0 {
			candidate, count = num, 1
		} else if num == candidate {
			count++
		} else {
			count--
		}



}	return candidate	}