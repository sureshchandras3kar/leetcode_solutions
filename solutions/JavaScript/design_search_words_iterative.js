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
        const queue = [[this.root, 0]];

        while (queue.length > 0) {
            const [node, index] = queue.shift();

            if (index === word.length) {
                if (node.isEndOfWord) {
                    return true;
                }
                continue;
            }

            let char = word[index];
            if (char === '.') {
                for (let child of Object.values(node.children)) {
                    queue.push([child, index + 1]);
                }
            } else {
                if (char in node.children) {
                    queue.push([node.children[char], index + 1]);
                }
            }
        }
        return false;
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
