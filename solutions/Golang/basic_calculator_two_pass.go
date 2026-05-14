package main

import (
	"fmt"
	"strconv"
	"unicode"
)

/*
Evaluate a mathematical expression with +, -, *, /, and parentheses.

Approach: Two-Pass with precedence handling
- First pass: convert infix to postfix notation (Reverse Polish Notation)
- Second pass: evaluate postfix expression using a stack
- Handles operator precedence (* and / before + and -)

Time: O(n) - two passes through string
Space: O(n) - stacks for postfix and evaluation
*/

func precedence(op rune) int {
	if op == '+' || op == '-' {
		return 1
	}
	if op == '*' || op == '/' {
		return 2
	}
	return 0
}

func basicCalculatorTwoPass(s string) int {
	if len(s) == 0 {
		return 0
	}

	// Tokenize the expression
	var tokens []interface{}
	i := 0
	runes := []rune(s)

	for i < len(runes) {
		if unicode.IsSpace(runes[i]) {
			i++
		} else if unicode.IsDigit(runes[i]) {
			num := 0
			for i < len(runes) && unicode.IsDigit(runes[i]) {
				num = num*10 + int(runes[i]-'0')
				i++
			}
			tokens = append(tokens, num)
		} else if runes[i] == '+' || runes[i] == '-' ||
			runes[i] == '*' || runes[i] == '/' ||
			runes[i] == '(' || runes[i] == ')' {
			tokens = append(tokens, runes[i])
			i++
		} else {
			i++
		}
	}

	// Convert infix to postfix (Shunting Yard algorithm)
	var output []interface{}
	var opStack []rune

	for _, token := range tokens {
		switch v := token.(type) {
		case int:
			output = append(output, v)
		case rune:
			if v == '(' {
				opStack = append(opStack, v)
			} else if v == ')' {
				for len(opStack) > 0 && opStack[len(opStack)-1] != '(' {
					output = append(output, opStack[len(opStack)-1])
					opStack = opStack[:len(opStack)-1]
				}
				if len(opStack) > 0 {
					opStack = opStack[:len(opStack)-1]  // Remove '('
				}
			} else if v == '+' || v == '-' || v == '*' || v == '/' {
				for len(opStack) > 0 &&
					opStack[len(opStack)-1] != '(' &&
					precedence(opStack[len(opStack)-1]) >= precedence(v) {
					output = append(output, opStack[len(opStack)-1])
					opStack = opStack[:len(opStack)-1]
				}
				opStack = append(opStack, v)
			}
		}
	}

	for len(opStack) > 0 {
		output = append(output, opStack[len(opStack)-1])
		opStack = opStack[:len(opStack)-1]
	}

	// Evaluate postfix expression
	var evalStack []int64
	for _, token := range output {
		switch v := token.(type) {
		case int:
			evalStack = append(evalStack, int64(v))
		case rune:
			if len(evalStack) >= 2 {
				b := evalStack[len(evalStack)-1]
				evalStack = evalStack[:len(evalStack)-1]
				a := evalStack[len(evalStack)-1]
				evalStack = evalStack[:len(evalStack)-1]

				switch v {
				case '+':
					evalStack = append(evalStack, a+b)
				case '-':
					evalStack = append(evalStack, a-b)
				case '*':
					evalStack = append(evalStack, a*b)
				case '/':
					evalStack = append(evalStack, a/b)
				}
			}
		}
	}

	if len(evalStack) > 0 {
		return int(evalStack[0])
	}
	return 0
}

func main() {
	fmt.Println(basicCalculatorTwoPass("1 + 1"))  // 2
	fmt.Println(basicCalculatorTwoPass(" 2-1 + 2 "))  // 3
	fmt.Println(basicCalculatorTwoPass("(1+(4+5+2)-3)+(6+8)"))  // 23
	fmt.Println(basicCalculatorTwoPass("2*(5+5*2)/3+(6/2*5)"))  // 17
}
