/*
LeetCode #208: Implement Trie (Prefix Tree)
Approach: Optimized with Word Storage
Time: O(m) for insert, search, startsWith
Space: O(N * m) where N = number of words
Benefit: Can retrieve all words with common prefix
*/

import java.util.HashMap;
import java.util.Map;
import java.util.ArrayList;
import java.util.List;

class TrieNodeOptimized {
    Map<Character, TrieNodeOptimized> children = new HashMap<>();
    boolean isEndOfWord = false;
    String word = null;  // Store complete word at end nodes
}

public class ImplementTrieOptimized {
    private TrieNodeOptimized root;

    public ImplementTrieOptimized() {
        root = new TrieNodeOptimized();
    }

    // Insert word with word storage. Time: O(m)
    public void insert(String word) {
        TrieNodeOptimized node = root;
        for (char c : word.toCharArray()) {
            if (!node.children.containsKey(c)) {
                node.children.put(c, new TrieNodeOptimized());
            }
            node = node.children.get(c);
        }
        node.isEndOfWord = true;
        node.word = word;  // Store complete word
    }

    // Search for exact word. Time: O(m)
    public boolean search(String word) {
        TrieNodeOptimized node = findNode(word);
        return node != null && node.isEndOfWord;
    }

    // Check if prefix exists. Time: O(m)
    public boolean startsWith(String prefix) {
        return findNode(prefix) != null;
    }

    private TrieNodeOptimized findNode(String prefix) {
        TrieNodeOptimized node = root;
        for (char c : prefix.toCharArray()) {
            if (!node.children.containsKey(c)) {
                return null;
            }
            node = node.children.get(c);
        }
        return node;
    }

    // Bonus: Get all words with given prefix. Time: O(n*m)
    public List<String> getAllWordsWithPrefix(String prefix) {
        List<String> result = new ArrayList<>();
        TrieNodeOptimized node = findNode(prefix);
        if (node == null) {
            return result;
        }
        dfsCollectWords(node, result);
        return result;
    }

    private void dfsCollectWords(TrieNodeOptimized node, List<String> result) {
        if (node.isEndOfWord && node.word != null) {
            result.add(node.word);
        }
        for (TrieNodeOptimized child : node.children.values()) {
            dfsCollectWords(child, result);
        }
    }

    // Usage
    public static void main(String[] args) {
        ImplementTrieOptimized trie = new ImplementTrieOptimized();
        trie.insert("apple");
        trie.insert("application");
        trie.insert("apply");

        System.out.println(trie.search("apple"));           // true
        System.out.println(trie.startsWith("app"));         // true
        System.out.println(trie.getAllWordsWithPrefix("app"));  // [apple, application, apply]
    }
}
