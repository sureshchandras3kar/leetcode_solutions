package main

import (
	"fmt"
	"strconv"
)

/**
 * Evaluate Reverse Polish Notation using recursion.
 *
 * Time: O(n) - process each token once
 * Space: O(n) - recursion stack depth
 */
type RPNEvaluator struct {
	tokens []string
	index  int
}

func (e *RPNEvaluator) helper() int64 {
	token := e.tokens[e.index]
	e.index--

	switch token {
	case "+":
		b := e.helper()
		a := e.helper()
		return a + b

	case "-":
		b := e.helper()
		a := e.helper()
		return a - b

	case "*":
		b := e.helper()
		a := e.helper()
		return a * b

	case "/":
		b := e.helper()
		a := e.helper()
		return a / b

	default:
		num, _ := strconv.ParseInt(token, 10, 64)
		return num
	}
}

func evaluateRpnRecursive(tokens []string) int {
	evaluator := &RPNEvaluator{
		tokens: tokens,
		index:  len(tokens) - 1,
	}
	return int(evaluator.helper())
}

// Test cases
func main() {
	fmt.Println(evaluateRpnRecursive([]string{"2", "1", "+", "3", "*"})) // 9
	fmt.Println(evaluateRpnRecursive([]string{"4", "13", "5", "/", "+"})) // 6
	fmt.Println(evaluateRpnRecursive([]string{"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"})) // 22
	fmt.Println(evaluateRpnRecursive([]string{"3", "4", "+"})) // 7
	fmt.Println(evaluateRpnRecursive([]string{"3", "4", "*"})) // 12
}
