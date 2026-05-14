#include <iostream>
#include <string>
#include <stack>
#include <vector>
#include <cctype>

int basicCalculatorTwoPass(std::string s) {
    /*
    Evaluate a mathematical expression with +, -, *, /, and parentheses.

    Approach: Two-Pass with precedence handling
    - First pass: convert infix to postfix notation (Reverse Polish Notation)
    - Second pass: evaluate postfix expression using a stack
    - Handles operator precedence (* and / before + and -)

    Time: O(n) - two passes through string
    Space: O(n) - stacks for postfix and evaluation
    */

    if (s.empty()) return 0;

    auto precedence = [](char op) -> int {
        if (op == '+' || op == '-') return 1;
        if (op == '*' || op == '/') return 2;
        return 0;
    };

    // Tokenize the expression
    std::vector<std::string> tokens;
    int i = 0;
    while (i < (int)s.length()) {
        if (isspace(s[i])) {
            i++;
        } else if (isdigit(s[i])) {
            int num = 0;
            while (i < (int)s.length() && isdigit(s[i])) {
                num = num * 10 + (s[i] - '0');
                i++;
            }
            tokens.push_back(std::to_string(num));
        } else if (s[i] == '+' || s[i] == '-' || s[i] == '*' ||
                   s[i] == '/' || s[i] == '(' || s[i] == ')') {
            tokens.push_back(std::string(1, s[i]));
            i++;
        } else {
            i++;
        }
    }

    // Convert infix to postfix (Shunting Yard algorithm)
    std::vector<std::string> output;
    std::stack<std::string> opStack;

    for (const auto& token : tokens) {
        if (isdigit(token[0])) {
            output.push_back(token);
        } else if (token == "(") {
            opStack.push(token);
        } else if (token == ")") {
            while (!opStack.empty() && opStack.top() != "(") {
                output.push_back(opStack.top());
                opStack.pop();
            }
            if (!opStack.empty()) {
                opStack.pop();  // Remove '('
            }
        } else if (token[0] == '+' || token[0] == '-' ||
                   token[0] == '*' || token[0] == '/') {
            while (!opStack.empty() && opStack.top() != "(" &&
                   precedence(opStack.top()[0]) >= precedence(token[0])) {
                output.push_back(opStack.top());
                opStack.pop();
            }
            opStack.push(token);
        }
    }

    while (!opStack.empty()) {
        output.push_back(opStack.top());
        opStack.pop();
    }

    // Evaluate postfix expression
    std::stack<long long> evalStack;
    for (const auto& token : output) {
        if (isdigit(token[0])) {
            evalStack.push(std::stoll(token));
        } else {
            long long b = evalStack.top(); evalStack.pop();
            long long a = evalStack.top(); evalStack.pop();
            if (token == "+") {
                evalStack.push(a + b);
            } else if (token == "-") {
                evalStack.push(a - b);
            } else if (token == "*") {
                evalStack.push(a * b);
            } else if (token == "/") {
                evalStack.push(a / b);
            }
        }
    }

    return evalStack.empty() ? 0 : evalStack.top();
}

int main() {
    std::cout << basicCalculatorTwoPass("1 + 1") << std::endl;  // 2
    std::cout << basicCalculatorTwoPass(" 2-1 + 2 ") << std::endl;  // 3
    std::cout << basicCalculatorTwoPass("(1+(4+5+2)-3)+(6+8)") << std::endl;  // 23
    std::cout << basicCalculatorTwoPass("2*(5+5*2)/3+(6/2*5)") << std::endl;  // 17
    return 0;
}
