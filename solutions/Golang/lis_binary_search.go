package main

import (
	"fmt"
	"sort"
)

/*
 * Find length of longest increasing subsequence using binary search O(n log n).
 *
 * Time Complexity: O(n log n)
 * Space Complexity: O(n)
 */
func lisBinarySearch(nums []int) int {
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

func main() {
	fmt.Println(lisBinarySearch([]int{10, 9, 2, 5, 3, 7, 101, 18}))  // 4
}
