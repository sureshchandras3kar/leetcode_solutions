"""
LeetCode #208: Implement Trie (Prefix Tree)
Approach: Trie Node Class with Hash Map
Time: O(m) for insert, search, startsWith where m is key length
Space: O(ALPHABET_SIZE * N) where N is number of keys
"""

class TrieNode:
    def __init__(self):
        # Store children as a dictionary {char: TrieNode}
        self.children = {}
        # Mark if this node represents end of a word
        self.is_end_of_word = False


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        """Insert a word into the trie. Time: O(m) where m = len(word)"""
        node = self.root
        for char in word:
            if char not in node.children:
                node.children[char] = TrieNode()
            node = node.children[char]
        node.is_end_of_word = True

    def search(self, word: str) -> bool:
        """Search for exact word. Time: O(m) where m = len(word)"""
        node = self.root
        for char in word:
            if char not in node.children:
                return False
            node = node.children[char]
        return node.is_end_of_word

    def startsWith(self, prefix: str) -> bool:
        """Check if any word starts with given prefix. Time: O(m)"""
        node = self.root
        for char in prefix:
            if char not in node.children:
                return False
            node = node.children[char]
        return True


# Usage
if __name__ == "__main__":
    trie = Trie()
    trie.insert("apple")
    print(trie.search("apple"))      # True
    print(trie.search("app"))        # False
    print(trie.startsWith("app"))    # True
    trie.insert("app")
    print(trie.search("app"))        # True
