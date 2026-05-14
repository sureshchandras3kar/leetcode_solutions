import java.util.*;

public class SubstringWithConcatenationOfAllWordsBruteForce {
    public static List<Integer> findSubstring(String s, String[] words) {
        List<Integer> result = new ArrayList<>();

        if (words == null || words.length == 0 || s == null || s.length() == 0) {
            return result;
        }

        int wordLen = words[0].length();
        int wordCount = words.length;
        int totalLen = wordLen * wordCount;

        // For each possible starting position in the string
        for (int i = 0; i <= s.length() - totalLen; i++) {
            // Extract the substring of exact length
            String substring = s.substring(i, i + totalLen);

            // Check if this substring can be formed by concatenating all words
            List<String> tempWords = new ArrayList<>(Arrays.asList(words));
            boolean valid = true;

            for (int j = 0; j < wordCount; j++) {
                String word = substring.substring(j * wordLen, (j + 1) * wordLen);
                if (tempWords.contains(word)) {
                    tempWords.remove(word);
                } else {
                    valid = false;
                    break;
                }
            }

            if (valid && tempWords.isEmpty()) {
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
