import java.util.Stack;
import java.util.HashMap;

public class Main {
    /**
     * Check if parentheses are balanced using a stack.
     *
     * Approach: Use a stack to match closing brackets with opening brackets.
     * - Push opening brackets onto the stack
     * - For each closing bracket, check if it matches the top of the stack
     * - At the end, the stack should be empty
     *
     * Time: O(n) - single pass through the string
     * Space: O(n) - stack size in worst case (all opening brackets)
     */
    public static boolean validParenthesesStack(String s) {
        Stack<Character> stack = new Stack<>();
        HashMap<Character, Character> bracketMap = new HashMap<>();
        bracketMap.put('(', ')');
        bracketMap.put('[', ']');
        bracketMap.put('{', '}');

        for (char c : s.toCharArray()) {
            if (bracketMap.containsKey(c)) {
                // Opening bracket - push to stack
                stack.push(c);
            } else {
                // Closing bracket - check if it matches
                if (stack.isEmpty() || bracketMap.get(stack.pop()) != c) {
                    return false;
                }
            }
        }

        // Stack should be empty (all brackets matched)
        return stack.isEmpty();
    }

    public static void main(String[] args) {
        // Test cases
        System.out.println(validParenthesesStack("()"));  // true
        System.out.println(validParenthesesStack("()[]{}"));  // true
        System.out.println(validParenthesesStack("(]"));  // false
        System.out.println(validParenthesesStack("([])"));  // true
        System.out.println(validParenthesesStack("{[]}"));  // true
        System.out.println(validParenthesesStack(""));  // true
        System.out.println(validParenthesesStack("("));  // false
        System.out.println(validParenthesesStack(")"));  // false
    }
}
