import java.util.*;

/**
 * Evaluate Reverse Polish Notation using a stack.
 *
 * Time: O(n) - process each token once
 * Space: O(n) - stack stores operands
 */
public class evaluate_reverse_polish_notation_stack {

    public static int evaluateRpnStack(String[] tokens) {
        Stack<Long> stack = new Stack<>();

        for (String token : tokens) {
            if (token.equals("+") || token.equals("-") || token.equals("*") || token.equals("/")) {
                // Pop two operands (order matters for - and /)
                long b = stack.pop();
                long a = stack.pop();

                long result;
                switch (token) {
                    case "+":
                        result = a + b;
                        break;
                    case "-":
                        result = a - b;
                        break;
                    case "*":
                        result = a * b;
                        break;
                    case "/":
                        // Truncate towards zero
                        result = a / b;
                        break;
                    default:
                        result = 0;
                }
                stack.push(result);
            } else {
                // It's a number
                stack.push(Long.parseLong(token));
            }
        }

        return (int) (long) stack.peek();
    }

    // Test cases
    public static void main(String[] args) {
        System.out.println(evaluateRpnStack(new String[]{"2", "1", "+", "3", "*"})); // 9
        System.out.println(evaluateRpnStack(new String[]{"4", "13", "5", "/", "+"})); // 6
        System.out.println(evaluateRpnStack(new String[]{"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"})); // 22
        System.out.println(evaluateRpnStack(new String[]{"3", "4", "+"})); // 7
        System.out.println(evaluateRpnStack(new String[]{"3", "4", "*"})); // 12
    }
}
