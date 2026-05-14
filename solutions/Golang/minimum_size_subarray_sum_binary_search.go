package main

import (
	"fmt"
	"sort"
)

func minSubArrayLenBinarySearch(target int, nums []int) int {
	prefix := []int{0}
	for _, num := range nums {
		prefix = append(prefix, prefix[len(prefix)-1]+num)
	}

	minLength := int(^uint(0) >> 1)

	for right := 1; right < len(prefix); right++ {
		needed := prefix[right] - target
		leftPos := binarySearchRightmost(prefix, needed, 0, right)
		left := leftPos - 1

		if left >= 0 && left < right {
			if right-left < minLength {
				minLength = right - left
			}
		}
	}

	if minLength == int(^uint(0)>>1) {
		return 0
	}
	return minLength
}

func binarySearchRightmost(arr []int, target int, lo int, hi int) int {
	result := sort.SearchInts(arr[lo:hi], target+1) + lo
	return result
}

func main() {
	nums := []int{2, 3, 1, 2, 4, 3}
	target := 7
	result := minSubArrayLenBinarySearch(target, nums)
	fmt.Println(result)  // 2
}
