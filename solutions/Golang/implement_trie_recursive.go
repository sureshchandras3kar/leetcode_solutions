package main

import "fmt"

type TrieNode struct {
	children    map[rune]*TrieNode
	isEndOfWord bool
}

type Trie struct {
	root *TrieNode
}

func NewTrie() *Trie {
	return &Trie{
		root: &TrieNode{
			children: make(map[rune]*TrieNode),
		},
	}
}

func (t *Trie) Insert(word string) {
	node := t.root
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

func (t *Trie) Search(word string) bool {
	node := t.findNode(word)
	return node != nil && node.isEndOfWord
}

func (t *Trie) StartsWith(prefix string) bool {
	return t.findNode(prefix) != nil
}

func (t *Trie) findNode(prefix string) *TrieNode {
	node := t.root
	for _, ch := range prefix {
		if child, exists := node.children[ch]; exists {
			node = child
		} else {
			return nil
		}
	}
	return node
}

func main() {
	trie := NewTrie()
	trie.Insert("apple")
	fmt.Println(trie.Search("apple"))    // true
	fmt.Println(trie.Search("app"))      // false
	fmt.Println(trie.StartsWith("app"))  // true
}
