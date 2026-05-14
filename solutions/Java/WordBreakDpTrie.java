import java.util.*;

class TrieNode {
    Map<Character, TrieNode> children = new HashMap<>();
    boolean is_end = false;
}

/**
 * Determine if string can be segmented using words from dictionary (Trie approach).
 *
 * Time Complexity: O(n*m)
 * Space Complexity: O(n + T)
 */
public class WordBreakDpTrie {
    public static boolean wordBreakDpTrie(String s, List<String> wordDict) {
        TrieNode root = new TrieNode();
        for (String word : wordDict) {
            TrieNode node = root;
            for (char ch : word.toCharArray()) {
                node.children.putIfAbsent(ch, new TrieNode());
                node = node.children.get(ch);
            }
            node.is_end = true;
        }

        boolean[] dp = new boolean[s.length() + 1];
        dp[0] = true;

        for (int i = 1; i <= s.length(); i++) {
            if (!dp[i]) {
                TrieNode node = root;
                for (int j = i - 1; j >= 0; j--) {
                    char ch = s.charAt(j);
                    if (!node.children.containsKey(ch)) {
                        break;
                    }
                    node = node.children.get(ch);
                    if (dp[j] && node.is_end) {
                        dp[i] = true;
                        break;
                    }
                }
            }
        }

        return dp[s.length()];
    }

    public static void main(String[] args) {
        System.out.println(wordBreakDpTrie("leetcode", Arrays.asList("leet", "code")));  // true
        System.out.println(wordBreakDpTrie("applepenapple", Arrays.asList("apple", "pen")));  // true
        System.out.println(wordBreakDpTrie("catsandog", Arrays.asList("cat", "cats", "and", "sand", "dog")));  // false
    }
}
