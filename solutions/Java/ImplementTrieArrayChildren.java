/*
LeetCode #208: Implement Trie (Prefix Tree)
Approach: Trie Node with Fixed-Size Array (26 letters)
Time: O(m) for insert, search, startsWith
Space: O(ALPHABET_SIZE * N) where ALPHABET_SIZE = 26
*/

public class ImplementTrieArrayChildren {
    private static final int ALPHABET_SIZE = 26;

    private static class TrieNode {
        TrieNode[] children = new TrieNode[ALPHABET_SIZE];
        boolean isEndOfWord = false;

        int charToIndex(char c) {
            return c - 'a';
        }
    }

    private TrieNode root;

    public ImplementTrieArrayChildren() {
        root = new TrieNode();
    }

    // Insert word into trie. Time: O(m)
    public void insert(String word) {
        TrieNode node = root;
        for (char c : word.toCharArray()) {
            int idx = node.charToIndex(c);
            if (node.children[idx] == null) {
                node.children[idx] = new TrieNode();
            }
            node = node.children[idx];
        }
        node.isEndOfWord = true;
    }

    // Search for exact word. Time: O(m)
    public boolean search(String word) {
        TrieNode node = root;
        for (char c : word.toCharArray()) {
            int idx = node.charToIndex(c);
            if (node.children[idx] == null) {
                return false;
            }
            node = node.children[idx];
        }
        return node.isEndOfWord;
    }

    // Check if any word starts with prefix. Time: O(m)
    public boolean startsWith(String prefix) {
        TrieNode node = root;
        for (char c : prefix.toCharArray()) {
            int idx = node.charToIndex(c);
            if (node.children[idx] == null) {
                return false;
            }
            node = node.children[idx];
        }
        return true;
    }

    // Usage
    public static void main(String[] args) {
        ImplementTrieArrayChildren trie = new ImplementTrieArrayChildren();
        trie.insert("apple");
        System.out.println(trie.search("apple"));      // true
        System.out.println(trie.search("app"));        // false
        System.out.println(trie.startsWith("app"));    // true
    }
}
