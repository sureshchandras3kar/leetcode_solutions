package main

import "fmt"

/*
 * Determine if string can be segmented using words from dictionary.
 *
 * Time Complexity: O(n²)
 * Space Complexity: O(n + m)
 */
func wordBreakDpSet(s string, wordDict []string) bool {
	wordSet := make(map[string]bool)
	for _, word := range wordDict {
		wordSet[word] = true
	}

	dp := make([]bool, len(s)+1)
	dp[0] = true

	for i := 1; i <= len(s); i++ {
		for j := 0; j < i; j++ {
			if dp[j] && wordSet[s[j:i]] {
				dp[i] = true
				break
			}
		}
	}

	return dp[len(s)]
}

func main() {
	fmt.Println(wordBreakDpSet("leetcode", []string{"leet", "code"}))  // true
	fmt.Println(wordBreakDpSet("applepenapple", []string{"apple", "pen"}))  // true
}
