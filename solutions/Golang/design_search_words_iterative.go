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
	type state struct {
		node  *TrieNode
		index int
	}

	queue := []state{{wd.root, 0}}

	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]

		if current.index == len(word) {
			if current.node.isEndOfWord {
				return true
			}
			continue
		}

		ch := rune(word[current.index])
		if ch == '.' {
			for _, child := range current.node.children {
				queue = append(queue, state{child, current.index + 1})
			}
		} else {
			if child, exists := current.node.children[ch]; exists {
				queue = append(queue, state{child, current.index + 1})
			}
		}
	}
	return false
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
