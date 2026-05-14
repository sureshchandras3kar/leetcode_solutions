package main

import "fmt"

/**
 * Check if parentheses are balanced using a stack.
 *
 * Approach: Use a stack to match closing brackets with opening brackets.
 * - Push opening brackets onto the stack
 * - For each closing bracket, check if it matches the top of the stack
 * - At the end, the stack should be empty
 *
 * Time: O(n) - single pass through the string
 * Space: O(n) - stack size in worst case (all opening brackets)
 */
func validParenthesesStack(s string) bool {
	stack := make([]rune, 0)
	bracketMap := map[rune]rune{
		'(': ')',
		'[': ']',
		'{': '}',
	}

	for _, c := range s {
		if val, ok := bracketMap[c]; ok {
			// Opening bracket - push to stack
			stack = append(stack, c)
		} else {
			// Closing bracket - check if it matches
			if len(stack) == 0 || bracketMap[stack[len(stack)-1]] != c {
				return false
			}
			stack = stack[:len(stack)-1]
		}
	}

	// Stack should be empty (all brackets matched)
	return len(stack) == 0
}

func main() {
	// Test cases
	fmt.Println(validParenthesesStack("()"))        // true
	fmt.Println(validParenthesesStack("()[]{}"))    // true
	fmt.Println(validParenthesesStack("(]"))        // false
	fmt.Println(validParenthesesStack("([])"))      // true
	fmt.Println(validParenthesesStack("{[]}"))      // true
	fmt.Println(validParenthesesStack(""))          // true
	fmt.Println(validParenthesesStack("("))         // false
	fmt.Println(validParenthesesStack(")"))         // false
}
