package main

import "fmt"

type TrieNode struct {
	children map[rune]*TrieNode
	word     string
}

func findWords(board [][]byte, words []string) []string {
	root := &TrieNode{children: make(map[rune]*TrieNode)}
	for _, word := range words {
		node := root
		for _, ch := range word {
			if _, exists := node.children[ch]; !exists {
				node.children[ch] = &TrieNode{children: make(map[rune]*TrieNode)}
			}
			node = node.children[ch]
		}
		node.word = word
	}

	result := []string{}

	var backtrack func(int, int, *TrieNode)
	backtrack = func(i, j int, node *TrieNode) {
		if i < 0 || i >= len(board) || j < 0 || j >= len(board[0]) {
			return
		}

		ch := rune(board[i][j])
		if _, exists := node.children[ch]; !exists {
			return
		}

		nextNode := node.children[ch]
		if nextNode.word != "" {
			result = append(result, nextNode.word)
			nextNode.word = ""
		}

		board[i][j] = '#'
		backtrack(i+1, j, nextNode)
		backtrack(i-1, j, nextNode)
		backtrack(i, j+1, nextNode)
		backtrack(i, j-1, nextNode)
		board[i][j] = byte(ch)
	}

	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[0]); j++ {
			backtrack(i, j, root)
		}
	}

	return result
}

func main() {
	board := [][]byte{
		{'o', 'a', 'a', 'n'},
		{'e', 't', 'a', 'e'},
		{'i', 'h', 'k', 'r'},
		{'i', 'f', 'l', 'v'},
	}
	words := []string{"oath", "pea", "eat", "rain"}
	fmt.Println(findWords(board, words))
}
