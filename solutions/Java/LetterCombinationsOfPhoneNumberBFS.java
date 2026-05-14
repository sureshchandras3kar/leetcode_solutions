import java.util.*;

class Solution {
    private Map<Character, String> mapping = new HashMap<>();

    public Solution() {
        mapping.put('2', "abc");
        mapping.put('3', "def");
        mapping.put('4', "ghi");
        mapping.put('5', "jkl");
        mapping.put('6', "mno");
        mapping.put('7', "pqrs");
        mapping.put('8', "tuv");
        mapping.put('9', "wxyz");
    }

    public List<String> letterCombinations(String digits) {
        List<String> result = new ArrayList<>();
        if (digits == null || digits.isEmpty()) return result;

        Queue<String> queue = new LinkedList<>();
        queue.add("");

        for (char digit : digits.toCharArray()) {
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                String current = queue.poll();
                for (char letter : mapping.get(digit).toCharArray()) {
                    queue.add(current + letter);
                }
            }
        }

        result.addAll(queue);
        return result;
    }
}
