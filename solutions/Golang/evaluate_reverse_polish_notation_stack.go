package main

import (
	"fmt"
	"strconv"
)

/**
 * Evaluate Reverse Polish Notation using a stack.
 *
 * Time: O(n) - process each token once
 * Space: O(n) - stack stores operands
 */
func evaluateRpnStack(tokens []string) int {
	stack := make([]int64, 0)

	for _, token := range tokens {
		switch token {
		case "+":
			b := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			a := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			stack = append(stack, a+b)

		case "-":
			b := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			a := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			stack = append(stack, a-b)

		case "*":
			b := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			a := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			stack = append(stack, a*b)

		case "/":
			b := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			a := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			stack = append(stack, a/b)

		default:
			// It's a number
			num, _ := strconv.ParseInt(token, 10, 64)
			stack = append(stack, num)
		}
	}

	return int(stack[0])
}

// Test cases
func main() {
	fmt.Println(evaluateRpnStack([]string{"2", "1", "+", "3", "*"})) // 9
	fmt.Println(evaluateRpnStack([]string{"4", "13", "5", "/", "+"})) // 6
	fmt.Println(evaluateRpnStack([]string{"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"})) // 22
	fmt.Println(evaluateRpnStack([]string{"3", "4", "+"})) // 7
	fmt.Println(evaluateRpnStack([]string{"3", "4", "*"})) // 12
}
