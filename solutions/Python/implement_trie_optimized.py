"""
LeetCode #208: Implement Trie (Prefix Tree)
Approach: Optimized with Word Storage
Time: O(m) for insert, search, startsWith
Space: O(N * m) where N = number of words, m = avg word length
Benefit: Can easily retrieve all words with common prefix
"""

class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_end_of_word = False
        self.word = None  # Store complete word at end nodes


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        """Insert word with word storage. Time: O(m)"""
        node = self.root
        for char in word:
            if char not in node.children:
                node.children[char] = TrieNode()
            node = node.children[char]
        node.is_end_of_word = True
        node.word = word  # Store complete word

    def search(self, word: str) -> bool:
        """Search for exact word. Time: O(m)"""
        node = self._find_node(word)
        return node is not None and node.is_end_of_word

    def startsWith(self, prefix: str) -> bool:
        """Check if prefix exists. Time: O(m)"""
        return self._find_node(prefix) is not None

    def _find_node(self, prefix: str) -> 'TrieNode':
        """Helper: find node at end of prefix. Time: O(m)"""
        node = self.root
        for char in prefix:
            if char not in node.children:
                return None
            node = node.children[char]
        return node

    def get_all_words_with_prefix(self, prefix: str) -> list:
        """Bonus: Get all words with given prefix. Time: O(n*m)"""
        node = self._find_node(prefix)
        if node is None:
            return []

        result = []
        self._dfs_collect_words(node, result)
        return result

    def _dfs_collect_words(self, node: 'TrieNode', result: list) -> None:
        """DFS to collect all words from node"""
        if node.is_end_of_word:
            result.append(node.word)
        for child in node.children.values():
            self._dfs_collect_words(child, result)


# Usage
if __name__ == "__main__":
    trie = Trie()
    trie.insert("apple")
    trie.insert("application")
    trie.insert("apply")

    print(trie.search("apple"))           # True
    print(trie.startsWith("app"))         # True
    words = trie.get_all_words_with_prefix("app")
    print(words)  # ['apple', 'application', 'apply']
