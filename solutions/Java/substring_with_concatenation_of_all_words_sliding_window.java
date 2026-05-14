import java.util.*;

public class SubstringWithConcatenationOfAllWordsSlidingWindow {
    public static List<Integer> findSubstring(String s, String[] words) {
        List<Integer> result = new ArrayList<>();

        if (words == null || words.length == 0 || s == null || s.length() == 0) {
            return result;
        }

        int wordLen = words[0].length();
        int wordCount = words.length;
        int totalLen = wordLen * wordCount;

        // Count frequency of each word
        Map<String, Integer> wordFreq = new HashMap<>();
        for (String word : words) {
            wordFreq.put(word, wordFreq.getOrDefault(word, 0) + 1);
        }

        // For each possible starting position
        for (int i = 0; i <= s.length() - totalLen; i++) {
            Map<String, Integer> windowFreq = new HashMap<>();

            // Extract and count words in this window
            for (int j = 0; j < wordCount; j++) {
                String word = s.substring(i + j * wordLen, i + (j + 1) * wordLen);
                windowFreq.put(word, windowFreq.getOrDefault(word, 0) + 1);
            }

            // Check if frequencies match
            if (windowFreq.equals(wordFreq)) {
                result.add(i);
            }
        }

        return result;
    }

    public static void main(String[] args) {
        // Example 1
        String s1 = "barfoothefoobarman";
        String[] words1 = {"foo", "bar"};
        System.out.println(findSubstring(s1, words1)); // [0, 9]

        // Example 2
        String s2 = "wordgoodgoodgoodbestword";
        String[] words2 = {"word", "good", "best", "word"};
        System.out.println(findSubstring(s2, words2)); // []

        // Example 3
        String s3 = "barfoofoobarthefoobarman";
        String[] words3 = {"bar", "foo", "the"};
        System.out.println(findSubstring(s3, words3)); // [6, 9, 12]
    }
}
