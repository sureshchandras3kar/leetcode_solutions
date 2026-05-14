class TrieNode {
    constructor() {
        this.children = {};
        this.isEndOfWord = false;
    }
}

class WordDictionary {
    constructor() {
        this.root = new TrieNode();
    }

    addWord(word) {
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
        return this.searchDFS(word, 0, this.root);
    }

    searchDFS(word, index, node) {
        if (index === word.length) {
            return node.isEndOfWord;
        }

        let char = word[index];
        if (char === '.') {
            for (let child of Object.values(node.children)) {
                if (this.searchDFS(word, index + 1, child)) {
                    return true;
                }
            }
            return false;
        } else {
            if (!(char in node.children)) {
                return false;
            }
            return this.searchDFS(word, index + 1, node.children[char]);
        }
    }
}

// Example usage
const wd = new WordDictionary();
wd.addWord("bad");
wd.addWord("dad");
wd.addWord("mad");
console.log(wd.search("pad"));  // false
console.log(wd.search("bad"));  // true
console.log(wd.search(".ad"));  // true
console.log(wd.search("b.."));  // true
