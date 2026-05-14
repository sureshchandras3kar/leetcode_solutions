package main

import (
	"fmt"
)

func findSubstringBruteForce(s string, words []string) []int {
	var result []int

	if len(words) == 0 || len(s) == 0 {
		return result
	}

	wordLen := len(words[0])
	wordCount := len(words)
	totalLen := wordLen * wordCount

	// For each possible starting position in the string
	for i := 0; i <= len(s)-totalLen; i++ {
		// Extract the substring of exact length
		substring := s[i : i+totalLen]

		// Check if this substring can be formed by concatenating all words
		tempWords := make([]string, len(words))
		copy(tempWords, words)
		valid := true

		for j := 0; j < wordCount; j++ {
			start := j * wordLen
			end := start + wordLen
			word := substring[start:end]

			found := false
			for k, w := range tempWords {
				if w == word {
					tempWords = append(tempWords[:k], tempWords[k+1:]...)
					found = true
					break
				}
			}

			if !found {
				valid = false
				break
			}
		}

		if valid && len(tempWords) == 0 {
			result = append(result, i)
		}
	}

	return result
}

func main() {
	// Example 1
	s1 := "barfoothefoobarman"
	words1 := []string{"foo", "bar"}
	fmt.Println(findSubstringBruteForce(s1, words1)) // [0, 9]

	// Example 2
	s2 := "wordgoodgoodgoodbestword"
	words2 := []string{"word", "good", "best", "word"}
	fmt.Println(findSubstringBruteForce(s2, words2)) // []

	// Example 3
	s3 := "barfoofoobarthefoobarman"
	words3 := []string{"bar", "foo", "the"}
	fmt.Println(findSubstringBruteForce(s3, words3)) // [6, 9, 12]
}
