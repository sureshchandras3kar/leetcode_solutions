class TrieNode {
    constructor() {
        this.children = {};
        this.isEndOfWord = false;
    }
}

class Trie {
    constructor() {
        this.root = new TrieNode();
    }

    insert(word) {
        let node = this.root;
        for (let char of word) {
            if (!(char in node.children)) {
                node.children[char] = new TrieNode();
            }
            node = node.children[char];
        }
        node.isEndOfWord = true;
    }

    search(word) {
        let node = this.findNode(word);
        return node !== null && node.isEndOfWord;
    }

    startsWith(prefix) {
        return this.findNode(prefix) !== null;
    }

    findNode(str) {
        let node = this.root;
        for (let char of str) {
            if (!(char in node.children)) {
                return null;
            }
            node = node.children[char];
        }
        return node;
    }
}

// Example usage
const trie = new Trie();
trie.insert("apple");
console.log(trie.search("apple"));    // true
console.log(trie.search("app"));      // false
console.log(trie.startsWith("app"));  // true
