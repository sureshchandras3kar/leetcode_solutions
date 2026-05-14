package main

import "sort"

func lengthOfLIS(nums []int) int {
	if len(nums) == 0 {
		return 0
	}

	tails := []int{}

	for _, num := range nums {
		pos := sort.SearchInts(tails, num)
		if pos == len(tails) {
			tails = append(tails, num)
		} else {
			tails[pos] = num
		}
	}

	return len(tails)
}
