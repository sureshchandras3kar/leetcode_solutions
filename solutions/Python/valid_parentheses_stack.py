from typing import List


def valid_parentheses_stack(s: str) -> bool:
    """
    Check if parentheses are balanced using a stack.

    Approach: Use a stack to match closing brackets with opening brackets.
    - Push opening brackets onto the stack
    - For each closing bracket, check if it matches the top of the stack
    - At the end, the stack should be empty

    Time: O(n) - single pass through the string
    Space: O(n) - stack size in worst case (all opening brackets)
    """
    stack = []
    bracket_map = {'(': ')', '[': ']', '{': '}'}

    for char in s:
        if char in bracket_map:
            # Opening bracket - push to stack
            stack.append(char)
        else:
            # Closing bracket - check if it matches
            if not stack or bracket_map[stack.pop()] != char:
                return False

    # Stack should be empty (all brackets matched)
    return len(stack) == 0


if __name__ == "__main__":
    # Test cases
    print(valid_parentheses_stack("()"))  # True
    print(valid_parentheses_stack("()[]{}"))  # True
    print(valid_parentheses_stack("(]"))  # False
    print(valid_parentheses_stack("([])"))  # True
    print(valid_parentheses_stack("{[]}"))  # True
    print(valid_parentheses_stack(""))  # True
    print(valid_parentheses_stack("("))  # False
    print(valid_parentheses_stack(")"))  # False
