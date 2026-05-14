package main

import (
	"fmt"
	"strconv"
	"strings"
)

func addBinaryIterative(a, b string) string {
	result := []string{}
	carry := 0
	i, j := len(a)-1, len(b)-1

	for i >= 0 || j >= 0 || carry > 0 {
		digitA := 0
		if i >= 0 {
			digitA, _ = strconv.Atoi(string(a[i]))
		}
		digitB := 0
		if j >= 0 {
			digitB, _ = strconv.Atoi(string(b[j]))
		}
		total := digitA + digitB + carry
		result = append([]string{strconv.Itoa(total % 2)}, result...)
		carry = total / 2
		i--
		j--
	}

	return strings.Join(result, "")
}

func main() {
	fmt.Println(addBinaryIterative("11", "1"))  // "100"
}
