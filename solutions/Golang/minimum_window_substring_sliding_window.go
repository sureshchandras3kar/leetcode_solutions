package main

import (
	"fmt"
)

func minWindowSlidingWindow(s string, t string) string {
	if s == "" || t == "" || len(s) < len(t) {
		return ""
	}

	// Dictionary to store frequency of characters in t
	dictT := make(map[rune]int)
	for _, char := range t {
		dictT[char]++
	}

	required := len(dictT)
	formed := 0

	windowCounts := make(map[rune]int)

	// ans: [window length, left, right]
	ansLen := len(s) + 1
	ansLeft, ansRight := 0, 0

	l := 0

	for r, char := range s {
		windowCounts[char]++

		if dictT[char] > 0 && windowCounts[char] == dictT[char] {
			formed++
		}

		for l <= r && formed == required {
			charL := rune(s[l])

			if r-l+1 < ansLen {
				ansLen = r - l + 1
				ansLeft = l
				ansRight = r
			}

			windowCounts[charL]--
			if dictT[charL] > 0 && windowCounts[charL] < dictT[charL] {
				formed--
			}

			l++
		}
	}

	if ansLen == len(s)+1 {
		return ""
	}
	return s[ansLeft : ansRight+1]
}

func main() {
	fmt.Println(minWindowSlidingWindow("ADOBECODEBANC", "ABC"))  // "BANC"
	fmt.Println(minWindowSlidingWindow("a", "a"))  // "a"
	fmt.Println(minWindowSlidingWindow("a", "aa"))  // ""
}
