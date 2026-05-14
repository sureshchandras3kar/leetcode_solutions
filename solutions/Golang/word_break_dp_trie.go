package main

import "fmt"

type TrieNode struct {
	children map[rune]*TrieNode
	is_end   bool
}

func NewTrieNode() *TrieNode {
	return &TrieNode{
		children: make(map[rune]*TrieNode),
		is_end:   false,
	}
}

/*
 * Determine if string can be segmented using words from dictionary (Trie approach).
 *
 * Time Complexity: O(n*m)
 * Space Complexity: O(n + T)
 */
func wordBreakDpTrie(s string, wordDict []string) bool {
	root := NewTrieNode()

	for _, word := range wordDict {
		node := root
		for _, ch := range word {
			if _, exists := node.children[ch]; !exists {
				node.children[ch] = NewTrieNode()
			}
			node = node.children[ch]
		}
		node.is_end = true
	}

	chars := []rune(s)
	dp := make([]bool, len(chars)+1)
	dp[0] = true

	for i := 1; i <= len(chars); i++ {
		if !dp[i] {
			node := root
			for j := i - 1; j >= 0; j-- {
				ch := chars[j]
				if next, exists := node.children[ch]; exists {
					node = next
					if dp[j] && node.is_end {
						dp[i] = true
						break
					}
				} else {
					break
				}
			}
		}
	}

	return dp[len(chars)]
}

func main() {
	fmt.Println(wordBreakDpTrie("leetcode", []string{"leet", "code"}))  // true
	fmt.Println(wordBreakDpTrie("applepenapple", []string{"apple", "pen"}))  // true
}
