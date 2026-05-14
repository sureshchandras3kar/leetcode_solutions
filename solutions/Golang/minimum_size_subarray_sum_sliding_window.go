package main

import (
	"fmt"
)

func minSubArrayLenSlidingWindow(target int, nums []int) int {
	left := 0
	currentSum := 0
	minLength := int(^uint(0) >> 1)

	for right := 0; right < len(nums); right++ {
		currentSum += nums[right]

		for currentSum >= target {
			if right-left+1 < minLength {
				minLength = right - left + 1
			}
			currentSum -= nums[left]
			left++
		}
	}

	if minLength == int(^uint(0)>>1) {
		return 0
	}
	return minLength
}

func main() {
	nums := []int{2, 3, 1, 2, 4, 3}
	target := 7
	result := minSubArrayLenSlidingWindow(target, nums)
	fmt.Println(result)  // 2
}
