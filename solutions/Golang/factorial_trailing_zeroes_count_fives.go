package main

import "fmt"

func factorialTrailingZeroesCountFives(n int) int {
	count := 0
	powerOf5 := 5
	for powerOf5 <= n {
		count += n / powerOf5
		powerOf5 *= 5
	}
	return count
}

func main() {
	fmt.Println(factorialTrailingZeroesCountFives(5))  // 1
	fmt.Println(factorialTrailingZeroesCountFives(25))  // 6
}
