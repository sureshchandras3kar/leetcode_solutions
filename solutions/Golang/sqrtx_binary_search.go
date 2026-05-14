package main

import "fmt"

func sqrtxBinarySearch(x int) int {
	if x < 2 {
		return x
	}

	left, right := 2, x/2
	for left <= right {
		mid := (left + right) / 2
		if mid*mid == x {
			return mid
		} else if mid*mid < x {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}

	return right
}

func main() {
	fmt.Println(sqrtxBinarySearch(4))  // 2
	fmt.Println(sqrtxBinarySearch(8))  // 2
}
