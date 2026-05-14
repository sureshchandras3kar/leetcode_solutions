def add_binary_string_simulation(a: str, b: str) -> str:
    """
    String simulation approach - simulate binary addition from right to left.
    Time: O(max(len(a), len(b)))
    Space: O(max(len(a), len(b))) for result
    """
    result = []
    carry = 0
    i = len(a) - 1
    j = len(b) - 1

    while i >= 0 or j >= 0 or carry:
        digit_a = int(a[i]) if i >= 0 else 0
        digit_b = int(b[j]) if j >= 0 else 0

        total = digit_a + digit_b + carry
        result.append(str(total % 2))
        carry = total // 2

        i -= 1
        j -= 1

    return ''.join(reversed(result))


print(add_binary_string_simulation("11", "1"))      # "100"
print(add_binary_string_simulation("1010", "1011")) # "10101"
print(add_binary_string_simulation("0", "0"))       # "0"
