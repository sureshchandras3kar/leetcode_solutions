package main

import (
	"fmt"
)

func minWindowBruteForce(s string, t string) string {
	if s == "" || t == "" || len(s) < len(t) {
		return ""
	}

	dictT := make(map[rune]int)
	for _, char := range t {
		dictT[char]++
	}

	minLen := len(s) + 1
	minStart := 0

	// Check all possible substrings
	for i := 0; i < len(s); i++ {
		for j := i + 1; j <= len(s); j++ {
			substring := s[i:j]

			substringCount := make(map[rune]int)
			for _, char := range substring {
				substringCount[char]++
			}

			// Verify if this substring is valid
			valid := true
			for char, count := range dictT {
				if substringCount[char] < count {
					valid = false
					break
				}
			}

			// Update minimum if this is a valid and shorter substring
			if valid && len(substring) < minLen {
				minLen = len(substring)
				minStart = i
			}
		}
	}

	if minLen == len(s)+1 {
		return ""
	}
	return s[minStart : minStart+minLen]
}

func main() {
	fmt.Println(minWindowBruteForce("ADOBECODEBANC", "ABC"))  // "BANC"
	fmt.Println(minWindowBruteForce("a", "a"))  // "a"
	fmt.Println(minWindowBruteForce("a", "aa"))  // ""
}
