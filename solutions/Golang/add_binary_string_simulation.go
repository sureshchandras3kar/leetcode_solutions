package main

import (
	"fmt"
	"strings"
)

/*
String simulation approach - simulate binary addition from right to left.
Time: O(max(len(a), len(b)))
Space: O(max(len(a), len(b))) for result
*/
func addBinaryStringSimulation(a string, b string) string {
	var result strings.Builder
	carry := 0
	i := len(a) - 1
	j := len(b) - 1

	for i >= 0 || j >= 0 || carry > 0 {
		digitA := 0
		if i >= 0 {
			digitA = int(a[i] - '0')
		}

		digitB := 0
		if j >= 0 {
			digitB = int(b[j] - '0')
		}

		total := digitA + digitB + carry
		result.WriteByte(byte('0' + (total % 2)))
		carry = total / 2

		i--
		j--
	}

	// Reverse the result
	str := result.String()
	runes := []rune(str)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

func main() {
	fmt.Println(addBinaryStringSimulation("11", "1"))      // "100"
	fmt.Println(addBinaryStringSimulation("1010", "1011")) // "10101"
	fmt.Println(addBinaryStringSimulation("0", "0"))       // "0"
}
