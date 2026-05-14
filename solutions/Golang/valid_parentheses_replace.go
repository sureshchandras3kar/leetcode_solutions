package main

import (
	"fmt"
	"strings"
)

/**
 * Check if parentheses are balanced by repeatedly removing matched pairs.
 *
 * Approach: Repeatedly remove consecutive pairs of matching brackets
 * until either the string is empty (valid) or no more pairs can be removed (invalid).
 *
 * Time: O(n²) - each pass removes at least 2 characters, up to n/2 passes
 * Space: O(n) - for temporary strings during replacement
 *
 * Note: This approach is simple to understand but slower than stack.
 */
func validParenthesesReplace(s string) bool {
	prev := ""
	for s != prev {
		prev = s
		s = strings.ReplaceAll(s, "()", "")
		s = strings.ReplaceAll(s, "[]", "")
		s = strings.ReplaceAll(s, "{}", "")
	}

	return len(s) == 0
}

func main() {
	// Test cases
	fmt.Println(validParenthesesReplace("()"))        // true
	fmt.Println(validParenthesesReplace("()[]{}"))    // true
	fmt.Println(validParenthesesReplace("(]"))        // false
	fmt.Println(validParenthesesReplace("([])"))      // true
	fmt.Println(validParenthesesReplace("{[]}"))      // true
	fmt.Println(validParenthesesReplace(""))          // true
	fmt.Println(validParenthesesReplace("("))         // false
	fmt.Println(validParenthesesReplace(")"))         // false
}
