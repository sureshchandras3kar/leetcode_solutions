class TrieNode {
    constructor() {
        this.children = {};
        this.word = null;
    }
}

class Solution {
    findWords(board, words) {
        const root = new TrieNode();
        for (let word of words) {
            let node = root;
            for (let char of word) {
                if (!(char in node.children)) {
                    node.children[char] = new TrieNode();
                }
                node = node.children[char];
            }
            node.word = word;
        }

        const result = [];

        const backtrack = (i, j, node) => {
            if (i < 0 || i >= board.length || j < 0 || j >= board[0].length) {
                return;
            }

            const char = board[i][j];
            if (!(char in node.children)) {
                return;
            }

            const nextNode = node.children[char];
            if (nextNode.word !== null) {
                result.push(nextNode.word);
                nextNode.word = null;
            }

            board[i][j] = '#';
            backtrack(i + 1, j, nextNode);
            backtrack(i - 1, j, nextNode);
            backtrack(i, j + 1, nextNode);
            backtrack(i, j - 1, nextNode);
            board[i][j] = char;
        };

        for (let i = 0; i < board.length; i++) {
            for (let j = 0; j < board[0].length; j++) {
                backtrack(i, j, root);
            }
        }

        return result;
    }
}

const sol = new Solution();
const board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]];
const words = ["oath","pea","eat","rain"];
console.log(sol.findWords(board, words));
