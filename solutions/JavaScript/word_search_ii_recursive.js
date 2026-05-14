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
        for (let i = 0; i < board.length; i++) {
            for (let j = 0; j < board[0].length; j++) {
                this.dfs(board, i, j, root, result);
            }
        }
        return result;
    }

    dfs(board, i, j, node, result) {
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
        const dirs = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        for (let [di, dj] of dirs) {
            const ni = i + di, nj = j + dj;
            if (ni >= 0 && ni < board.length && nj >= 0 && nj < board[0].length) {
                this.dfs(board, ni, nj, nextNode, result);
            }
        }
        board[i][j] = char;
    }
}

const sol = new Solution();
const board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]];
const words = ["oath","pea","eat","rain"];
console.log(sol.findWords(board, words));
