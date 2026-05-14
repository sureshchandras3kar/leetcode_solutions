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
	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[0]); j++ {
			dfs(board, i, j, root, &result)
		}
	}
	return result
}

func dfs(board [][]byte, i, j int, node *TrieNode, result *[]string) {
	ch := rune(board[i][j])
	if _, exists := node.children[ch]; !exists {
		return
	}

	nextNode := node.children[ch]
	if nextNode.word != "" {
		*result = append(*result, nextNode.word)
		nextNode.word = ""
	}

	board[i][j] = '#'
	dirs := [][]int{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}
	for _, dir := range dirs {
		ni, nj := i+dir[0], j+dir[1]
		if ni >= 0 && ni < len(board) && nj >= 0 && nj < len(board[0]) {
			dfs(board, ni, nj, nextNode, result)
		}
	}
	board[i][j] = byte(ch)
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
