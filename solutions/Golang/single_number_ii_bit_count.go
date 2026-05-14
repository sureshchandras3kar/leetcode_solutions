package main

import "fmt"

func singleNumberIIBitCount(nums []int) int {
	bitCounts := make([]int, 32)
	for _, num := range nums {
		for i := 0; i < 32; i++ {
			if (num & (1 << i)) != 0 {
				bitCounts[i]++
			}
		}
	}

	result := 0
	for i := 0; i < 32; i++ {
		if bitCounts[i]%3 != 0 {
			result |= (1 << i)
		}
	}

	if result >= (1 << 31) {
		result -= (1 << 32)
	}
	return result
}

func main() {
	fmt.Println(singleNumberIIBitCount([]int{2, 2, 3, 2}))  // 3
}
