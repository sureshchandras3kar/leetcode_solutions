package main

import (
	"fmt"
	"strconv"
)

/*
Bit operations approach - convert to uint64, add, convert back to binary.
Time: O(max(len(a), len(b)))
Space: O(max(len(a), len(b))) for result
*/
func addBinaryBitOperations(a string, b string) string {
	numA, _ := strconv.ParseUint(a, 2, 64)
	numB, _ := strconv.ParseUint(b, 2, 64)
	total := numA + numB
	return strconv.FormatUint(total, 2)
}

func main() {
	fmt.Println(addBinaryBitOperations("11", "1"))      // "100"
	fmt.Println(addBinaryBitOperations("1010", "1011")) // "10101"
	fmt.Println(addBinaryBitOperations("0", "0"))       // "0"
}
