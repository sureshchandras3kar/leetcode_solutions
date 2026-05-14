package main

import (
	"fmt"
	"sort"
)

func threeSumHashSet(nums []int) [][]int {
	sort.Ints(nums)
	result := [][]int{}
	n := len(nums)

	for i := 0; i < n-2; i++ {
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}
		seen := make(map[int]bool)
		for j := i + 1; j < n; j++ {
			need := -(nums[i] + nums[j])
			if seen[need] {
				triplet := []int{nums[i], need, nums[j]}
				duplicate := false
				for _, existing := range result {
					if existing[0] == triplet[0] && existing[1] == triplet[1] && existing[2] == triplet[2] {
						duplicate = true
						break
					}
				}
				if !duplicate {
					result = append(result, triplet)
				}
			}
			seen[nums[j]] = true
		}
	}

	return result
}

func main() {
	nums := []int{-1, 0, 1, 2, -1, -4}
	result := threeSumHashSet(nums)
	fmt.Println(result) // [[-1 -1 2] [-1 0 1]]
}
