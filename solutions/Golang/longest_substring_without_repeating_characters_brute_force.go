package main

import "fmt"

func longestSubstringWithoutRepeatingCharactersBruteForce(s string) int {
	maxLength := 0

	for i := 0; i < len(s); i++ {
		charSet := make(map[rune]bool)
		for j, c := range s[i:] {
			if charSet[c] {
				break
			}
			charSet[c] = true
			if i+j+1 > maxLength {
				maxLength = i + j + 1
			}
		}
	}

	return maxLength
}

func main() {
	s := "abcabcbb"
	result := longestSubstringWithoutRepeatingCharactersBruteForce(s)
	fmt.Println(result)
}
