from typing import List


class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_end = False


def word_break_dp_trie(s: str, word_dict: List[str]) -> bool:
    """
    Determine if string can be segmented using words from dictionary (Trie approach).

    Time Complexity: O(n*m) where n = len(s), m = max word length
    Space Complexity: O(n) for DP array + O(T) for Trie where T = total characters in dictionary

    Build a Trie from the dictionary, then use DP with Trie for faster lookups.
    dp[i] = True if s[0:i] can be segmented.
    Instead of checking all substrings (O(n²)), we traverse the Trie (O(m)).
    """
    # Build Trie
    root = TrieNode()
    for word in word_dict:
        node = root
        for char in word:
            if char not in node.children:
                node.children[char] = TrieNode()
            node = node.children[char]
        node.is_end = True

    dp = [False] * (len(s) + 1)
    dp[0] = True

    for i in range(1, len(s) + 1):
        if not dp[i]:
            # Try to find a word ending at position i
            node = root
            for j in range(i - 1, -1, -1):
                char = s[j]
                if char not in node.children:
                    break
                node = node.children[char]
                if dp[j] and node.is_end:
                    dp[i] = True
                    break

    return dp[len(s)]


if __name__ == "__main__":
    print(word_break_dp_trie("leetcode", ["leet", "code"]))           # True
    print(word_break_dp_trie("applepenapple", ["apple", "pen"]))      # True
    print(word_break_dp_trie("catsandog", ["cat", "cats", "and", "sand", "dog"]))  # False
