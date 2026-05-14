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

        self.result = []
        visited = set()

        def backtrack(i, j, node):
            if i < 0 or i >= len(board) or j < 0 or j >= len(board[0]):
                return
            if (i, j) in visited:
                return

            char = board[i][j]
            if char not in node.children:
                return

            visited.add((i, j))
            next_node = node.children[char]

            if next_node.word is not None:
                self.result.append(next_node.word)
                next_node.word = None

            for di, dj in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
                backtrack(i + di, j + dj, next_node)

            visited.remove((i, j))

        for i in range(len(board)):
            for j in range(len(board[0])):
                backtrack(i, j, root)

        return self.result


sol = Solution()
board = [["o", "a", "a", "n"], ["e", "t", "a", "e"], ["i", "h", "k", "r"], ["i", "f", "l", "v"]]
words = ["oath", "pea", "eat", "rain"]
print(sol.findWords(board, words))  # ['eat', 'oath']
