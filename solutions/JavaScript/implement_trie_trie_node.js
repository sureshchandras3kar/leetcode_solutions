/*
LeetCode #208: Implement Trie (Prefix Tree)
Approach: Trie Node Class with Map
Time: O(m) for insert, search, startsWith where m is key length
Space: O(ALPHABET_SIZE * N) where N is number of keys
*/

class TrieNode {
    constructor() {
        // Store children as Map {char: TrieNode}
        this.children = new Map();
        // Mark if this node represents end of a word
        this.isEndOfWord = false;
    }
}

class Trie {
    constructor() {
        this.root = new TrieNode();
    }

    // Insert a word into the trie. Time: O(m)
    insert(word) {
        let node = this.root;
        for (const char of word) {
            if (!node.children.has(char)) {
                node.children.set(char, new TrieNode());
            }
            node = node.children.get(char);
        }
        node.isEndOfWord = true;
    }

    // Search for exact word. Time: O(m)
    search(word) {
        let node = this.root;
        for (const char of word) {
            if (!node.children.has(char)) {
                return false;
            }
            node = node.children.get(char);
        }
        return node.isEndOfWord;
    }

    // Check if any word starts with given prefix. Time: O(m)
    startsWith(prefix) {
        let node = this.root;
        for (const char of prefix) {
            if (!node.children.has(char)) {
                return false;
            }
            node = node.children.get(char);
        }
        return true;
    }
}

// Usage
const trie = new Trie();
trie.insert("apple");
console.log(trie.search("apple"));      // true
console.log(trie.search("app"));        // false
console.log(trie.startsWith("app"));    // true
trie.insert("app");
console.log(trie.search("app"));        // true
