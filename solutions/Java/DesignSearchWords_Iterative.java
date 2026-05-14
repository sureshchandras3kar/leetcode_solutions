import java.util.HashMap;
import java.util.Map;
import java.util.Queue;
import java.util.LinkedList;

class TrieNode {
    Map<Character, TrieNode> children = new HashMap<>();
    boolean isEndOfWord = false;
}

class WordDictionary {
    private TrieNode root;

    public WordDictionary() {
        root = new TrieNode();
    }

    public void addWord(String word) {
        TrieNode node = root;
        for (char c : word.toCharArray()) {
            node.children.putIfAbsent(c, new TrieNode());
            node = node.children.get(c);
        }
        node.isEndOfWord = true;
    }

    public boolean search(String word) {
        Queue<Map.Entry<TrieNode, Integer>> queue = new LinkedList<>();
        queue.offer(new java.util.AbstractMap.SimpleEntry<>(root, 0));

        while (!queue.isEmpty()) {
            Map.Entry<TrieNode, Integer> entry = queue.poll();
            TrieNode node = entry.getKey();
            int index = entry.getValue();

            if (index == word.length()) {
                if (node.isEndOfWord) {
                    return true;
                }
                continue;
            }

            char ch = word.charAt(index);
            if (ch == '.') {
                for (TrieNode child : node.children.values()) {
                    queue.offer(new java.util.AbstractMap.SimpleEntry<>(child, index + 1));
                }
            } else {
                if (node.children.containsKey(ch)) {
                    queue.offer(new java.util.AbstractMap.SimpleEntry<>(node.children.get(ch), index + 1));
                }
            }
        }
        return false;
    }
}

public class DesignSearchWords_Iterative {
    public static void main(String[] args) {
        WordDictionary wd = new WordDictionary();
        wd.addWord("bad");
        wd.addWord("dad");
        wd.addWord("mad");
        System.out.println(wd.search("pad"));  // false
        System.out.println(wd.search("bad"));  // true
        System.out.println(wd.search(".ad"));  // true
        System.out.println(wd.search("b.."));  // true
    }
}
