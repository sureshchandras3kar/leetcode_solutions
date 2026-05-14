package main

import "fmt"

func longestSubstringWithoutRepeatingCharactersSlidingWindow(s string) int {
	charIndex := make(map[rune]int)
	maxLength := 0
	left := 0

	for right, c := range s {
		if prevIndex, ok := charIndex[c]; ok && prevIndex >= left {
			left = prevIndex + 1
		}
		charIndex[c] = right
		if right-left+1 > maxLength {
			maxLength = right - left + 1
		}
	}

	return maxLength
}

func main() {
	s := "abcabcbb"
	result := longestSubstringWithoutRepeatingCharactersSlidingWindow(s)
	fmt.Println(result)
}
