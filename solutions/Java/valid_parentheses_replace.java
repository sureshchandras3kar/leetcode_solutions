public class Main {
    /**
     * Check if parentheses are balanced by repeatedly removing matched pairs.
     *
     * Approach: Repeatedly remove consecutive pairs of matching brackets
     * until either the string is empty (valid) or no more pairs can be removed (invalid).
     *
     * Time: O(n²) - each pass removes at least 2 characters, up to n/2 passes
     * Space: O(n) - for temporary strings during replacement
     *
     * Note: This approach is simple to understand but slower than stack.
     */
    public static boolean validParenthesesReplace(String s) {
        String prev;
        do {
            prev = s;
            s = s.replace("()", "").replace("[]", "").replace("{}", "");
        } while (!s.equals(prev));

        return s.length() == 0;
    }

    public static void main(String[] args) {
        // Test cases
        System.out.println(validParenthesesReplace("()"));  // true
        System.out.println(validParenthesesReplace("()[]{}"));  // true
        System.out.println(validParenthesesReplace("(]"));  // false
        System.out.println(validParenthesesReplace("([])"));  // true
        System.out.println(validParenthesesReplace("{[]}"));  // true
        System.out.println(validParenthesesReplace(""));  // true
        System.out.println(validParenthesesReplace("("));  // false
        System.out.println(validParenthesesReplace(")"));  // false
    }
}
