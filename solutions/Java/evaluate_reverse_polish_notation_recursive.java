import java.util.*;

/**
 * Evaluate Reverse Polish Notation using recursion.
 *
 * Time: O(n) - process each token once
 * Space: O(n) - recursion stack depth
 */
public class evaluate_reverse_polish_notation_recursive {

    private String[] tokens;
    private int index;

    private long helper() {
        String token = tokens[index--];

        if (token.equals("+") || token.equals("-") || token.equals("*") || token.equals("/")) {
            // Process in reverse order for recursion (right operand first)
            long b = helper();
            long a = helper();

            switch (token) {
                case "+":
                    return a + b;
                case "-":
                    return a - b;
                case "*":
                    return a * b;
                case "/":
                    return a / b;
                default:
                    return 0;
            }
        } else {
            return Long.parseLong(token);
        }
    }

    public int evaluateRpnRecursive(String[] t) {
        tokens = t;
        index = t.length - 1;
        return (int) helper();
    }

    // Test cases
    public static void main(String[] args) {
        evaluate_reverse_polish_notation_recursive sol = new evaluate_reverse_polish_notation_recursive();

        System.out.println(sol.evaluateRpnRecursive(new String[]{"2", "1", "+", "3", "*"})); // 9
        System.out.println(sol.evaluateRpnRecursive(new String[]{"4", "13", "5", "/", "+"})); // 6
        System.out.println(sol.evaluateRpnRecursive(new String[]{"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"})); // 22
        System.out.println(sol.evaluateRpnRecursive(new String[]{"3", "4", "+"})); // 7
        System.out.println(sol.evaluateRpnRecursive(new String[]{"3", "4", "*"})); // 12
    }
}
