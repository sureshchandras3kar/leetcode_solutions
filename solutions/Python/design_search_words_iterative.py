from collections import deque


class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_end_of_word = False


class WordDictionary:
    def __init__(self):
        self.root = TrieNode()

    def addWord(self, word: str) -> None:
        node = self.root
        for char in word:
            if char not in node.children:
                node.children[char] = TrieNode()
            node = node.children[char]
        node.is_end_of_word = True

    def search(self, word: str) -> bool:
        queue = deque([(self.root, 0)])

        while queue:
            node, index = queue.popleft()

            if index == len(word):
                if node.is_end_of_word:
                    return True
                continue

            char = word[index]
            if char == '.':
                for child in node.children.values():
                    queue.append((child, index + 1))
            else:
                if char in node.children:
                    queue.append((node.children[char], index + 1))

        return False


# Example usage
wd = WordDictionary()
wd.addWord("bad")
wd.addWord("dad")
wd.addWord("mad")
print(wd.search("pad"))  # False
print(wd.search("bad"))  # True
print(wd.search(".ad"))  # True
print(wd.search("b.."))  # True
