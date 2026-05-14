import java.util.HashMap;
import java.util.Map;

public class MinimumWindowSubstringBruteForce {
    public static String minWindowBruteForce(String s, String t) {
        if (s == null || s.isEmpty() || t == null || t.isEmpty() || s.length() < t.length()) {
            return "";
        }

        Map<Character, Integer> dictT = new HashMap<>();
        for (char c : t.toCharArray()) {
            dictT.put(c, dictT.getOrDefault(c, 0) + 1);
        }

        int minLen = Integer.MAX_VALUE;
        int minStart = 0;

        // Check all possible substrings
        for (int i = 0; i < s.length(); i++) {
            for (int j = i + 1; j <= s.length(); j++) {
                String substring = s.substring(i, j);

                Map<Character, Integer> substringCount = new HashMap<>();
                for (char c : substring.toCharArray()) {
                    substringCount.put(c, substringCount.getOrDefault(c, 0) + 1);
                }

                // Verify if this substring is valid
                boolean valid = true;
                for (Map.Entry<Character, Integer> entry : dictT.entrySet()) {
                    if (substringCount.getOrDefault(entry.getKey(), 0) < entry.getValue()) {
                        valid = false;
                        break;
                    }
                }

                // Update minimum if this is a valid and shorter substring
                if (valid && substring.length() < minLen) {
                    minLen = substring.length();
                    minStart = i;
                }
            }
        }

        return minLen == Integer.MAX_VALUE ? "" : s.substring(minStart, minStart + minLen);
    }

    public static void main(String[] args) {
        System.out.println(minWindowBruteForce("ADOBECODEBANC", "ABC"));  // "BANC"
        System.out.println(minWindowBruteForce("a", "a"));  // "a"
        System.out.println(minWindowBruteForce("a", "aa"));  // ""
    }
}
