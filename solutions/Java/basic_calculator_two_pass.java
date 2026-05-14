import java.util.Stack;
import java.util.List;
import java.util.ArrayList;

class BasicCalculatorTwoPass {
    /**
     * Evaluate a mathematical expression with +, -, *, /, and parentheses.
     *
     * Approach: Two-Pass with precedence handling
     * - First pass: convert infix to postfix notation (Reverse Polish Notation)
     * - Second pass: evaluate postfix expression using a stack
     * - Handles operator precedence (* and / before + and -)
     *
     * Time: O(n) - two passes through string
     * Space: O(n) - stacks for postfix and evaluation
     */

    private static int precedence(char op) {
        if (op == '+' || op == '-') return 1;
        if (op == '*' || op == '/') return 2;
        return 0;
    }

    public static int basicCalculatorTwoPass(String s) {
        if (s == null || s.isEmpty()) return 0;

        // Tokenize the expression
        List<String> tokens = new ArrayList<>();
        int i = 0;
        while (i < s.length()) {
            if (Character.isWhitespace(s.charAt(i))) {
                i++;
            } else if (Character.isDigit(s.charAt(i))) {
                int num = 0;
                while (i < s.length() && Character.isDigit(s.charAt(i))) {
                    num = num * 10 + (s.charAt(i) - '0');
                    i++;
                }
                tokens.add(String.valueOf(num));
            } else if ("+-*/()".indexOf(s.charAt(i)) != -1) {
                tokens.add(String.valueOf(s.charAt(i)));
                i++;
            } else {
                i++;
            }
        }

        // Convert infix to postfix (Shunting Yard algorithm)
        List<String> output = new ArrayList<>();
        Stack<String> opStack = new Stack<>();

        for (String token : tokens) {
            if (token.charAt(0) >= '0' && token.charAt(0) <= '9') {
                output.add(token);
            } else if (token.equals("(")) {
                opStack.push(token);
            } else if (token.equals(")")) {
                while (!opStack.isEmpty() && !opStack.peek().equals("(")) {
                    output.add(opStack.pop());
                }
                if (!opStack.isEmpty()) {
                    opStack.pop();  // Remove '('
                }
            } else if ("+-*/".indexOf(token.charAt(0)) != -1) {
                while (!opStack.isEmpty() && !opStack.peek().equals("(") &&
                       precedence(opStack.peek().charAt(0)) >= precedence(token.charAt(0))) {
                    output.add(opStack.pop());
                }
                opStack.push(token);
            }
        }

        while (!opStack.isEmpty()) {
            output.add(opStack.pop());
        }

        // Evaluate postfix expression
        Stack<Long> evalStack = new Stack<>();
        for (String token : output) {
            if (token.charAt(0) >= '0' && token.charAt(0) <= '9') {
                evalStack.push(Long.parseLong(token));
            } else {
                long b = evalStack.pop();
                long a = evalStack.pop();
                switch (token.charAt(0)) {
                    case '+':
                        evalStack.push(a + b);
                        break;
                    case '-':
                        evalStack.push(a - b);
                        break;
                    case '*':
                        evalStack.push(a * b);
                        break;
                    case '/':
                        evalStack.push(a / b);
                        break;
                }
            }
        }

        return evalStack.isEmpty() ? 0 : evalStack.pop().intValue();
    }

    public static void main(String[] args) {
        System.out.println(basicCalculatorTwoPass("1 + 1"));  // 2
        System.out.println(basicCalculatorTwoPass(" 2-1 + 2 "));  // 3
        System.out.println(basicCalculatorTwoPass("(1+(4+5+2)-3)+(6+8)"));  // 23
        System.out.println(basicCalculatorTwoPass("2*(5+5*2)/3+(6/2*5)"));  // 17
    }
}
