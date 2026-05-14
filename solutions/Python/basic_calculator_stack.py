def basic_calculator_stack(s: str) -> int:
    """
    Evaluate a mathematical expression with +, -, *, /, and parentheses.

    Approach: Recursive stack-based parser
    - Use helper function to parse expression level (handles + and -)
    - Use another helper to parse term level (handles * and /)
    - Parentheses trigger recursive parsing of nested sub-expressions
    - This respects operator precedence naturally: term > expression

    Time: O(n) - single pass through string
    Space: O(d) - recursion depth equals nesting level
    """
    if not s:
        return 0

    def parse_number(s, i):
        """Extract a number starting at index i."""
        while i < len(s) and s[i].isspace():
            i += 1
        num = 0
        while i < len(s) and s[i].isdigit():
            num = num * 10 + int(s[i])
            i += 1
        return num, i

    def parse_factor(s, i):
        """Parse a factor: number or (expression)."""
        while i < len(s) and s[i].isspace():
            i += 1

        if s[i] == '(':
            i += 1  # Skip '('
            result, i = parse_expression(s, i)
            while i < len(s) and s[i].isspace():
                i += 1
            i += 1  # Skip ')'
            return result, i
        else:
            return parse_number(s, i)

    def parse_term(s, i):
        """Parse multiplication and division (higher precedence)."""
        result, i = parse_factor(s, i)

        while i < len(s):
            while i < len(s) and s[i].isspace():
                i += 1
            if i >= len(s) or s[i] not in '*/':
                break

            op = s[i]
            i += 1
            operand, i = parse_factor(s, i)

            if op == '*':
                result *= operand
            else:  # op == '/'
                result = int(result / operand)

        return result, i

    def parse_expression(s, i):
        """Parse addition and subtraction (lower precedence)."""
        result, i = parse_term(s, i)

        while i < len(s):
            while i < len(s) and s[i].isspace():
                i += 1
            if i >= len(s) or s[i] not in '+-':
                break

            op = s[i]
            i += 1
            operand, i = parse_term(s, i)

            if op == '+':
                result += operand
            else:  # op == '-'
                result -= operand

        return result, i

    result, _ = parse_expression(s, 0)
    return result


# Test cases
if __name__ == "__main__":
    print(basic_calculator_stack("1 + 1"))  # 2
    print(basic_calculator_stack(" 2-1 + 2 "))  # 3
    print(basic_calculator_stack("(1+(4+5+2)-3)+(6+8)"))  # 23
    print(basic_calculator_stack("2*(5+5*2)/3+(6/2*5)"))  # 17
