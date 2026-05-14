package main

import (
	"fmt"
)

func findSubstringWindowApproach(s string, words []string) []int {
	var result []int

	if len(words) == 0 || len(s) == 0 {
		return result
	}

	wordLen := len(words[0])
	wordCount := len(words)
	totalLen := wordLen * wordCount

	// Count frequency of each word
	wordFreq := make(map[string]int)
	for _, word := range words {
		wordFreq[word]++
	}

	// For each possible starting position
	for i := 0; i <= len(s)-totalLen; i++ {
		windowFreq := make(map[string]int)

		// Extract and count words in this window
		for j := 0; j < wordCount; j++ {
			start := i + j*wordLen
			end := start + wordLen
			word := s[start:end]
			windowFreq[word]++
		}

		// Check if frequencies match
		if mapsEqual(windowFreq, wordFreq) {
			result = append(result, i)
		}
	}

	return result
}

// Helper function to compare two maps
func mapsEqual(m1, m2 map[string]int) bool {
	if len(m1) != len(m2) {
		return false
	}
	for key, val := range m1 {
		if m2[key] != val {
			return false
		}
	}
	return true
}

func main() {
	// Example 1
	s1 := "barfoothefoobarman"
	words1 := []string{"foo", "bar"}
	fmt.Println(findSubstringWindowApproach(s1, words1)) // [0, 9]

	// Example 2
	s2 := "wordgoodgoodgoodbestword"
	words2 := []string{"word", "good", "best", "word"}
	fmt.Println(findSubstringWindowApproach(s2, words2)) // []

	// Example 3
	s3 := "barfoofoobarthefoobarman"
	words3 := []string{"bar", "foo", "the"}
	fmt.Println(findSubstringWindowApproach(s3, words3)) // [6, 9, 12]
}
