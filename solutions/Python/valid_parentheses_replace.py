def valid_parentheses_replace(s: str) -> bool:
    """
    Check if parentheses are balanced by repeatedly removing matched pairs.

    Approach: Repeatedly remove consecutive pairs of matching brackets
    until either the string is empty (valid) or no more pairs can be removed (invalid).

    Time: O(n²) - each pass removes at least 2 characters, up to n/2 passes
    Space: O(n) - for temporary string during replacement

    Note: This approach is simple to understand but slower than stack.
    """
    while "()" in s or "[]" in s or "{}" in s:
        s = s.replace("()", "").replace("[]", "").replace("{}", "")

    return len(s) == 0


if __name__ == "__main__":
    # Test cases
    print(valid_parentheses_replace("()"))  # True
    print(valid_parentheses_replace("()[]{}"))  # True
    print(valid_parentheses_replace("(]"))  # False
    print(valid_parentheses_replace("([])"))  # True
    print(valid_parentheses_replace("{[]}"))  # True
    print(valid_parentheses_replace(""))  # True
    print(valid_parentheses_replace("("))  # False
    print(valid_parentheses_replace(")"))  # False
