package main

import "fmt"

func plusOneIterate(digits []int) []int {
	carry := 1
	for i := len(digits) - 1; i >= 0; i-- {
		digits[i] += carry
		if digits[i] < 10 {
			return digits
		}
		digits[i] = 0
	}

	result := make([]int, len(digits)+1)
	result[0] = 1
	copy(result[1:], digits)
	return result
}

func main() {
	fmt.Println(plusOneIterate([]int{1, 2, 3}))  // [1 2 4]
	fmt.Println(plusOneIterate([]int{9, 9, 9}))  // [1 0 0 0]
}
