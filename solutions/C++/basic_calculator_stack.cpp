#include <iostream>
#include <string>
#include <cctype>

class Calculator {
private:
    std::string s;
    int pos;

    int parseNumber() {
        while (pos < (int)s.length() && isspace(s[pos])) pos++;
        int num = 0;
        while (pos < (int)s.length() && isdigit(s[pos])) {
            num = num * 10 + (s[pos] - '0');
            pos++;
        }
        return num;
    }

    int parseFactor() {
        while (pos < (int)s.length() && isspace(s[pos])) pos++;
        if (s[pos] == '(') {
            pos++;  // Skip '('
            int result = parseExpression();
            while (pos < (int)s.length() && isspace(s[pos])) pos++;
            pos++;  // Skip ')'
            return result;
        } else {
            return parseNumber();
        }
    }

    int parseTerm() {
        int result = parseFactor();
        while (pos < (int)s.length()) {
            while (pos < (int)s.length() && isspace(s[pos])) pos++;
            if (pos >= (int)s.length() || (s[pos] != '*' && s[pos] != '/')) break;
            char op = s[pos];
            pos++;
            int operand = parseFactor();
            if (op == '*') {
                result *= operand;
            } else {
                result = result / operand;
            }
        }
        return result;
    }

    int parseExpression() {
        int result = parseTerm();
        while (pos < (int)s.length()) {
            while (pos < (int)s.length() && isspace(s[pos])) pos++;
            if (pos >= (int)s.length() || (s[pos] != '+' && s[pos] != '-')) break;
            char op = s[pos];
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

public:
    int calculate(std::string expr) {
        s = expr;
        pos = 0;
        return parseExpression();
    }
};

int basicCalculatorStack(std::string s) {
    /*
    Evaluate a mathematical expression with +, -, *, /, and parentheses.

    Approach: Recursive descent parser
    - Parse expression level (+ and -)
    - Parse term level (* and /)
    - Parse factor level (numbers and parentheses)
    - Respects operator precedence naturally

    Time: O(n) - single pass through string
    Space: O(d) - recursion depth equals nesting level
    */
    if (s.empty()) return 0;
    Calculator calc;
    return calc.calculate(s);
}

int main() {
    std::cout << basicCalculatorStack("1 + 1") << std::endl;  // 2
    std::cout << basicCalculatorStack(" 2-1 + 2 ") << std::endl;  // 3
    std::cout << basicCalculatorStack("(1+(4+5+2)-3)+(6+8)") << std::endl;  // 23
    std::cout << basicCalculatorStack("2*(5+5*2)/3+(6/2*5)") << std::endl;  // 25
    return 0;
}
