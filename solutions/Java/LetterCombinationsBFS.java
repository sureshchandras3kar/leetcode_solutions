import java.util.*;

public class LetterCombinationsBFS {
    /**
     * Generate all letter combinations using BFS (iterative).
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

        Queue<String> queue = new LinkedList<>();
        queue.offer("");

        for (char digit : digits.toCharArray()) {
            int size = queue.size();
            String letters = phoneMap.get(digit);

            // Process all current combinations
            for (int i = 0; i < size; i++) {
                String current = queue.poll();

                // Create a new combination for each letter
                for (char letter : letters.toCharArray()) {
                    queue.offer(current + letter);
                }
            }
        }

        return new ArrayList<>(queue);
    }

    public static void main(String[] args) {
        LetterCombinationsBFS sol = new LetterCombinationsBFS();
        System.out.println(sol.letterCombinations("23"));
        // Output: [ad, ae, af, bd, be, bf, cd, ce, cf]
    }
}
