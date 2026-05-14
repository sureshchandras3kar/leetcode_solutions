/*
LeetCode #208: Implement Trie (Prefix Tree)
Approach: Trie Node with Plain Object Children
Time: O(m) for insert, search, startsWith
Space: O(ALPHABET_SIZE * N)
Note: Object is faster than Map in JavaScript for most use cases
*/

class TrieNode {
    constructor() {
        // Use plain object for children (faster than Map in JS)
        this.children = {};
        this.isEndOfWord = false;
    }
}

class Trie {
    constructor() {
        this.root = new TrieNode();
    }

    // Insert word into trie. Time: O(m)
    insert(word) {
        let node = this.root;
        for (const char of word) {
            if (!(char in node.children)) {
                node.children[char] = new TrieNode();
            }
            node = node.children[char];
        }
        node.isEndOfWord = true;
    }

    // Search for exact word. Time: O(m)
    search(word) {
        let node = this.root;
        for (const char of word) {
            if (!(char in node.children)) {
                return false;
            }
            node = node.children[char];
        }
        return node.isEndOfWord;
    }

    // Check if any word starts with prefix. Time: O(m)
    startsWith(prefix) {
        let node = this.root;
        for (const char of prefix) {
            if (!(char in node.children)) {
                return false;
            }
            node = node.children[char];
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
