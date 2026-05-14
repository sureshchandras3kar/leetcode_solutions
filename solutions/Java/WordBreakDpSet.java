import java.util.*;

/**
 * Determine if string can be segmented using words from dictionary.
 *
 * Time Complexity: O(n²)
 * Space Complexity: O(n + m)
 */
public class WordBreakDpSet {
    public static boolean wordBreakDpSet(String s, List<String> wordDict) {
        Set<String> wordSet = new HashSet<>(wordDict);
        boolean[] dp = new boolean[s.length() + 1];
        dp[0] = true;

        for (int i = 1; i <= s.length(); i++) {
            for (int j = 0; j < i; j++) {
                if (dp[j] && wordSet.contains(s.substring(j, i))) {
                    dp[i] = true;
                    break;
                }
            }
        }

        return dp[s.length()];
    }

    public static void main(String[] args) {
        System.out.println(wordBreakDpSet("leetcode", Arrays.asList("leet", "code")));  // true
        System.out.println(wordBreakDpSet("applepenapple", Arrays.asList("apple", "pen")));  // true
        System.out.println(wordBreakDpSet("catsandog", Arrays.asList("cat", "cats", "and", "sand", "dog")));  // false
    }
}
