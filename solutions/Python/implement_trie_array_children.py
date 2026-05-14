"""
LeetCode #208: Implement Trie (Prefix Tree)
Approach: Trie Node with Fixed-Size Array Children (26 letters)
Time: O(m) for insert, search, startsWith
Space: O(ALPHABET_SIZE * N) where ALPHABET_SIZE = 26
"""

class TrieNode:
    def __init__(self):
        # Fixed array for 26 lowercase letters
        self.children = [None] * 26
        self.is_end_of_word = False

    def _char_to_index(self, char: str) -> int:
        """Convert char to index (a=0, b=1, ..., z=25)"""
        return ord(char) - ord('a')


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        """Insert word into trie. Time: O(m)"""
        node = self.root
        for char in word:
            idx = node._char_to_index(char)
            if node.children[idx] is None:
                node.children[idx] = TrieNode()
            node = node.children[idx]
        node.is_end_of_word = True

    def search(self, word: str) -> bool:
        """Search for exact word. Time: O(m)"""
        node = self.root
        for char in word:
            idx = node._char_to_index(char)
            if node.children[idx] is None:
                return False
            node = node.children[idx]
        return node.is_end_of_word

    def startsWith(self, prefix: str) -> bool:
        """Check if any word starts with prefix. Time: O(m)"""
        node = self.root
        for char in prefix:
            idx = node._char_to_index(char)
            if node.children[idx] is None:
                return False
            node = node.children[idx]
        return True


# Usage
if __name__ == "__main__":
    trie = Trie()
    trie.insert("apple")
    print(trie.search("apple"))      # True
    print(trie.search("app"))        # False
    print(trie.startsWith("app"))    # True
