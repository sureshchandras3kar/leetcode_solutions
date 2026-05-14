package main

import "fmt"

type TrieNode struct {
	children    map[rune]*TrieNode
	isEndOfWord bool
}

type WordDictionary struct {
	root *TrieNode
}

func NewWordDictionary() *WordDictionary {
	return &WordDictionary{
		root: &TrieNode{
			children: make(map[rune]*TrieNode),
		},
	}
}

func (wd *WordDictionary) AddWord(word string) {
	node := wd.root
	for _, ch := range word {
		if _, exists := node.children[ch]; !exists {
			node.children[ch] = &TrieNode{
				children: make(map[rune]*TrieNode),
			}
		}
		node = node.children[ch]
	}
	node.isEndOfWord = true
}

func (wd *WordDictionary) Search(word string) bool {
	return wd.searchDFS(word, 0, wd.root)
}

func (wd *WordDictionary) searchDFS(word string, index int, node *TrieNode) bool {
	if index == len(word) {
		return node.isEndOfWord
	}

	ch := rune(word[index])
	if ch == '.' {
		for _, child := range node.children {
			if wd.searchDFS(word, index+1, child) {
				return true
			}
		}
		return false
	} else {
		if child, exists := node.children[ch]; exists {
			return wd.searchDFS(word, index+1, child)
		}
		return false
	}
}

func main() {
	wd := NewWordDictionary()
	wd.AddWord("bad")
	wd.AddWord("dad")
	wd.AddWord("mad")
	fmt.Println(wd.Search("pad"))  // false
	fmt.Println(wd.Search("bad"))  // true
	fmt.Println(wd.Search(".ad"))  // true
	fmt.Println(wd.Search("b.."))  // true
}
