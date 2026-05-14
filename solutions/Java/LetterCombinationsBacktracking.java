import java.util.*;

public class LetterCombinationsBacktracking {
    /**
     * Generate all letter combinations using backtracking (recursive).
     * Time: O(4^n), Space: O(4^n) for result
     */
    public List<String> letterCombinations(String digits) {
        if (digits.isEmpty()) return new ArrayList<>();

        Map<Character, String> phoneMap = new HashMap<>();
        phoneMap.put('2', "abc");
        phoneMap.put('3', "def");
        phoneMap.put('4', "ghi");
        phoneMap.put('5', "jkl");
        phoneMap.put('6', "mno");
        phoneMap.put('7', "pqrs");
        phoneMap.put('8', "tuv");
        phoneMap.put('9', "wxyz");

        List<String> result = new ArrayList<>();
        backtrack(digits, 0, new StringBuilder(), result, phoneMap);
        return result;
    }

    private void backtrack(String digits, int index, StringBuilder current,
                          List<String> result, Map<Character, String> phoneMap) {
        // Base case: we've processed all digits
        if (index == digits.length()) {
            result.add(current.toString());
            return;
        }

        // Get the letters that the current digit maps to
        char currentDigit = digits.charAt(index);
        String letters = phoneMap.get(currentDigit);

        // Try each letter
        for (char letter : letters.toCharArray()) {
            current.append(letter);
            backtrack(digits, index + 1, current, result, phoneMap);
            current.deleteCharAt(current.length() - 1);
        }
    }

    public static void main(String[] args) {
        LetterCombinationsBacktracking sol = new LetterCombinationsBacktracking();
        System.out.println(sol.letterCombinations("23"));
        // Output: [ad, ae, af, bd, be, bf, cd, ce, cf]
    }
}
