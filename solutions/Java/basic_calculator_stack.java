class BasicCalculatorStack {
    private String s;
    private int pos;

    /**
     * Evaluate a mathematical expression with +, -, *, /, and parentheses.
     *
     * Approach: Recursive descent parser
     * - Parse expression level (+ and -)
     * - Parse term level (* and /)
     * - Parse factor level (numbers and parentheses)
     * - Respects operator precedence naturally
     *
     * Time: O(n) - single pass through string
     * Space: O(d) - recursion depth equals nesting level
     */

    private int parseNumber() {
        while (pos < s.length() && Character.isWhitespace(s.charAt(pos))) pos++;
        int num = 0;
        while (pos < s.length() && Character.isDigit(s.charAt(pos))) {
            num = num * 10 + (s.charAt(pos) - '0');
            pos++;
        }
        return num;
    }

    private int parseFactor() {
        while (pos < s.length() && Character.isWhitespace(s.charAt(pos))) pos++;
        if (s.charAt(pos) == '(') {
            pos++;  // Skip '('
            int result = parseExpression();
            while (pos < s.length() && Character.isWhitespace(s.charAt(pos))) pos++;
            pos++;  // Skip ')'
            return result;
        } else {
            return parseNumber();
        }
    }

    private int parseTerm() {
        int result = parseFactor();
        while (pos < s.length()) {
            while (pos < s.length() && Character.isWhitespace(s.charAt(pos))) pos++;
            if (pos >= s.length() || (s.charAt(pos) != '*' && s.charAt(pos) != '/')) break;
            char op = s.charAt(pos);
            pos++;
            int operand = parseFactor();
            if (op == '*') {
                result *= operand;
            } else {
                result /= operand;
            }
        }
        return result;
    }

    private int parseExpression() {
        int result = parseTerm();
        while (pos < s.length()) {
            while (pos < s.length() && Character.isWhitespace(s.charAt(pos))) pos++;
            if (pos >= s.length() || (s.charAt(pos) != '+' && s.charAt(pos) != '-')) break;
            char op = s.charAt(pos);
            pos++;
            int operand = parseTerm();
            if (op == '+') {
                result += operand;
            } else {
                result -= operand;
            }
        }
        return result;
    }

    public int calculate(String expr) {
        s = expr;
        pos = 0;
        return parseExpression();
    }

    public static void main(String[] args) {
        BasicCalculatorStack calc = new BasicCalculatorStack();
        System.out.println(calc.calculate("1 + 1"));  // 2
        System.out.println(calc.calculate(" 2-1 + 2 "));  // 3
        System.out.println(calc.calculate("(1+(4+5+2)-3)+(6+8)"));  // 23
        System.out.println(calc.calculate("2*(5+5*2)/3+(6/2*5)"));  // 25
    }
}
