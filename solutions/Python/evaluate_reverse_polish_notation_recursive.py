from typing import List


def evaluate_rpn_recursive(tokens: List[str]) -> int:
    """
    Evaluate Reverse Polish Notation using recursion with a shared index.

    Time: O(n) - process each token once
    Space: O(n) - recursion stack depth
    """
    index = [len(tokens) - 1]  # Use list to allow modification in nested function
    operators = {'+', '-', '*', '/'}

    def helper() -> int:
        token = tokens[index[0]]
        index[0] -= 1

        if token in operators:
            # Process in reverse order for recursion (right operand first)
            b = helper()
            a = helper()

            if token == '+':
                return a + b
            elif token == '-':
                return a - b
            elif token == '*':
                return a * b
            else:  # token == '/'
                return int(a / b)
        else:
            return int(token)

    return helper()


# Test cases
if __name__ == "__main__":
    print(evaluate_rpn_recursive(["2", "1", "+", "3", "*"]))  # 9
    print(evaluate_rpn_recursive(["4", "13", "5", "/", "+"]))  # 6
    print(evaluate_rpn_recursive(["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]))  # 22
    print(evaluate_rpn_recursive(["3", "4", "+"]))  # 7
    print(evaluate_rpn_recursive(["3", "4", "*"]))  # 12
