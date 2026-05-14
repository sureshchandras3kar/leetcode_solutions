from typing import List


def evaluate_rpn_stack(tokens: List[str]) -> int:
    """
    Evaluate Reverse Polish Notation using a stack.

    Time: O(n) - process each token once
    Space: O(n) - stack stores operands
    """
    stack = []
    operators = {'+', '-', '*', '/'}

    for token in tokens:
        if token in operators:
            # Pop two operands (order matters for - and /)
            b = stack.pop()
            a = stack.pop()

            if token == '+':
                result = a + b
            elif token == '-':
                result = a - b
            elif token == '*':
                result = a * b
            else:  # token == '/'
                # Truncate towards zero (Python's int() does this)
                result = int(a / b)

            stack.append(result)
        else:
            # It's a number
            stack.append(int(token))

    return stack[0]


# Test cases
if __name__ == "__main__":
    print(evaluate_rpn_stack(["2", "1", "+", "3", "*"]))  # 9
    print(evaluate_rpn_stack(["4", "13", "5", "/", "+"]))  # 6
    print(evaluate_rpn_stack(["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]))  # 22
    print(evaluate_rpn_stack(["3", "4", "+"]))  # 7
    print(evaluate_rpn_stack(["3", "4", "*"]))  # 12
