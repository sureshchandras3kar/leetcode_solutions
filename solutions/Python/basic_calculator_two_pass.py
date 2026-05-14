def basic_calculator_two_pass(s: str) -> int:
    """
    Evaluate a mathematical expression with +, -, *, /, and parentheses.

    Approach: Two-Pass with precedence handling
    - First pass: convert infix to postfix notation (Reverse Polish Notation)
    - Second pass: evaluate postfix expression using a stack
    - Handles operator precedence (* and / before + and -)

    Time: O(n) - two passes through string
    Space: O(n) - stacks for postfix and evaluation
    """
    if not s:
        return 0

    def precedence(op: str) -> int:
        if op in "+-":
            return 1
        elif op in "*/":
            return 2
        return 0

    # Tokenize the expression
    tokens = []
    i = 0
    while i < len(s):
        if s[i].isspace():
            i += 1
        elif s[i].isdigit():
            num = 0
            while i < len(s) and s[i].isdigit():
                num = num * 10 + int(s[i])
                i += 1
            tokens.append(num)
        elif s[i] in "+-*/()":
            tokens.append(s[i])
            i += 1
        else:
            i += 1

    # Convert infix to postfix (Shunting Yard algorithm)
    output = []
    operator_stack = []

    for token in tokens:
        if isinstance(token, int):
            output.append(token)
        elif token == '(':
            operator_stack.append(token)
        elif token == ')':
            while operator_stack and operator_stack[-1] != '(':
                output.append(operator_stack.pop())
            if operator_stack:
                operator_stack.pop()  # Remove '('
        elif token in "+-*/":
            while (operator_stack and
                   operator_stack[-1] != '(' and
                   precedence(operator_stack[-1]) >= precedence(token)):
                output.append(operator_stack.pop())
            operator_stack.append(token)

    while operator_stack:
        output.append(operator_stack.pop())

    # Evaluate postfix expression
    stack = []
    for token in output:
        if isinstance(token, int):
            stack.append(token)
        else:
            b = stack.pop()
            a = stack.pop()
            if token == '+':
                stack.append(a + b)
            elif token == '-':
                stack.append(a - b)
            elif token == '*':
                stack.append(a * b)
            elif token == '/':
                # Integer division toward zero
                stack.append(int(a / b))

    return stack[0] if stack else 0


# Test cases
if __name__ == "__main__":
    print(basic_calculator_two_pass("1 + 1"))  # 2
    print(basic_calculator_two_pass(" 2-1 + 2 "))  # 3
    print(basic_calculator_two_pass("(1+(4+5+2)-3)+(6+8)"))  # 23
    print(basic_calculator_two_pass("2*(5+5*2)/3+(6/2*5)"))  # 17
