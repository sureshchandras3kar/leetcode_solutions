def is_palindrome(x: int) -> bool:
    if x < 0:
        return False
    if x == 0:
        return True
    if x % 10 == 0:
        return False
    num_reversed = 0
    original_x = x
    while original_x > 0:
        last_digit = original_x % 10
        num_reversed = num_reversed * 10 + last_digit
        original_x //= 10
    return x == num_reversed

print(is_palindrome(121))  # True
print(is_palindrome(-121))  # False
print(is_palindrome(10))  # False
