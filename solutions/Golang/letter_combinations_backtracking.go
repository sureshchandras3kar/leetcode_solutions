package main

import "fmt"

func letterCombinationsBacktracking(digits string) []string {
	if digits == "" {
		return []string{}
	}

	phoneMap := map[byte]string{
		'2': "abc",
		'3': "def",
		'4': "ghi",
		'5': "jkl",
		'6': "mno",
		'7': "pqrs",
		'8': "tuv",
		'9': "wxyz",
	}

	var result []string

	var backtrack func(int, string)
	backtrack = func(index int, current string) {
		// Base case: we've processed all digits
		if index == len(digits) {
			result = append(result, current)
			return
		}

		// Get the letters that the current digit maps to
		currentDigit := digits[index]
		letters := phoneMap[currentDigit]

		// Try each letter
		for _, letter := range letters {
			backtrack(index+1, current+string(letter))
		}
	}

	backtrack(0, "")
	return result
}

func main() {
	fmt.Println(letterCombinationsBacktracking("23"))
	// Output: [ad ae af bd be bf cd ce cf]
}
