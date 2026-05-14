package main

import "fmt"
import "math"

func powxnFastExponentiation(x float64, n int) float64 {
	if n == 0 {
		return 1.0
	}

	N := int64(n)
	if N < 0 {
		x = 1 / x
		N = -N
	}

	result := 1.0
	for N > 0 {
		if N%2 == 1 {
			result *= x
		}
		x *= x
		N /= 2
	}

	return result
}

func main() {
	fmt.Println(powxnFastExponentiation(2.0, 10))  // 1024
}
