/*
LeetCode #208: Implement Trie (Prefix Tree)
Approach: Trie Node Class with HashMap
Time: O(m) for insert, search, startsWith where m is key length
Space: O(ALPHABET_SIZE * N) where N is number of keys
*/

import java.util.HashMap;
import java.util.Map;

class TrieNode {
    // Store children as HashMap {char: TrieNode}
    Map<Character, TrieNode> children = new HashMap<>();
    // Mark if this node represents end of a word
    boolean isEndOfWord = false;
}

public class ImplementTrieTrieNode {
    private TrieNode root;

    public ImplementTrieTrieNode() {
        root = new TrieNode();
    }

    // Insert a word into the trie. Time: O(m)
    public void insert(String word) {
        TrieNode node = root;
        for (char c : word.toCharArray()) {
            if (!node.children.containsKey(c)) {
                node.children.put(c, new TrieNode());
            }
            node = node.children.get(c);
        }
        node.isEndOfWord = true;
    }

    // Search for exact word. Time: O(m)
    public boolean search(String word) {
        TrieNode node = root;
        for (char c : word.toCharArray()) {
            if (!node.children.containsKey(c)) {
                return false;
            }
            node = node.children.get(c);
        }
        return node.isEndOfWord;
    }

    // Check if any word starts with given prefix. Time: O(m)
    public boolean startsWith(String prefix) {
        TrieNode node = root;
        for (char c : prefix.toCharArray()) {
            if (!node.children.containsKey(c)) {
                return false;
            }
            node = node.children.get(c);
        }
        return true;
    }

    // Usage
    public static void main(String[] args) {
        ImplementTrieTrieNode trie = new ImplementTrieTrieNode();
        trie.insert("apple");
        System.out.println(trie.search("apple"));      // true
        System.out.println(trie.search("app"));        // false
        System.out.println(trie.startsWith("app"));    // true
        trie.insert("app");
        System.out.println(trie.search("app"));        // true
    }
}
