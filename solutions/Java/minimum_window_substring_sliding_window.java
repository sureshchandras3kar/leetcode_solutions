import java.util.HashMap;
import java.util.Map;

public class MinimumWindowSubstringSlidingWindow {
    public static String minWindowSlidingWindow(String s, String t) {
        if (s == null || s.isEmpty() || t == null || t.isEmpty() || s.length() < t.length()) {
            return "";
        }

        // Dictionary to store frequency of characters in t
        Map<Character, Integer> dictT = new HashMap<>();
        for (char c : t.toCharArray()) {
            dictT.put(c, dictT.getOrDefault(c, 0) + 1);
        }

        int required = dictT.size();
        int formed = 0;

        Map<Character, Integer> windowCounts = new HashMap<>();

        // ans array: {window length, left, right}
        int ansLen = Integer.MAX_VALUE;
        int ansLeft = 0, ansRight = 0;

        int l = 0;

        for (int r = 0; r < s.length(); r++) {
            char charR = s.charAt(r);
            windowCounts.put(charR, windowCounts.getOrDefault(charR, 0) + 1);

            if (dictT.containsKey(charR) && windowCounts.get(charR) == dictT.get(charR)) {
                formed++;
            }

            while (l <= r && formed == required) {
                char charL = s.charAt(l);

                if (r - l + 1 < ansLen) {
                    ansLen = r - l + 1;
                    ansLeft = l;
                    ansRight = r;
                }

                windowCounts.put(charL, windowCounts.get(charL) - 1);
                if (dictT.containsKey(charL) && windowCounts.get(charL) < dictT.get(charL)) {
                    formed--;
                }

                l++;
            }
        }

        return ansLen == Integer.MAX_VALUE ? "" : s.substring(ansLeft, ansRight + 1);
    }

    public static void main(String[] args) {
        System.out.println(minWindowSlidingWindow("ADOBECODEBANC", "ABC"));  // "BANC"
        System.out.println(minWindowSlidingWindow("a", "a"));  // "a"
        System.out.println(minWindowSlidingWindow("a", "aa"));  // ""
    }
}
