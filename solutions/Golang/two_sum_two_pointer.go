package main

import (
	"fmt"
	"sort"
)

func twoSumTwoPointer(nums []int, target int) []int {
	sorted := make([]int, len(nums))
	copy(sorted, nums)
	sort.Ints(sorted)
	left, right := 0, len(sorted)-1
	for left < right {
		currentSum := sorted[left] + sorted[right]
		if currentSum == target {
			return []int{sorted[left], sorted[right]}
		} else if currentSum < target {
			left++
		} else {
			right--
		}
	}
	return []int{}
}

func main() {
	nums := []int{2, 7, 11, 15}
	target := 9
	result := twoSumTwoPointer(nums, target)
	fmt.Println(result)
}
