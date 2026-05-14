package main

import "fmt"

func singleNumberXor(nums []int) int {
	result := 0
	for _, num := range nums {
		result ^= num
	}
	return result
}

func main() {
	fmt.Println(singleNumberXor([]int{2, 2, 1}))  // 1
	fmt.Println(singleNumberXor([]int{4, 1, 2, 1, 2}))  // 4
}
