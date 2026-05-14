from typing import List


class TrieNode:
    def __init__(self):
        self.children = {}
        self.word = None


class Solution:
    def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
        root = TrieNode()
        for word in words:
            node = root
            for char in word:
                if char not in node.children:
                    node.children[char] = TrieNode()
                node = node.children[char]
            node.word = word

        result = []
        for i in range(len(board)):
            for j in range(len(board[0])):
                self.dfs(board, i, j, root, result)
        return result

    def dfs(self, board, i, j, node, result):
        char = board[i][j]
        if char not in node.children:
            return

        next_node = node.children[char]
        if next_node.word is not None:
            result.append(next_node.word)
            next_node.word = None

        board[i][j] = '#'
        for di, dj in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
            ni, nj = i + di, j + dj
            if 0 <= ni < len(board) and 0 <= nj < len(board[0]):
                self.dfs(board, ni, nj, next_node, result)
        board[i][j] = char


sol = Solution()
board = [["o", "a", "a", "n"], ["e", "t", "a", "e"], ["i", "h", "k", "r"], ["i", "f", "l", "v"]]
words = ["oath", "pea", "eat", "rain"]
print(sol.findWords(board, words))  # ['eat', 'oath']
