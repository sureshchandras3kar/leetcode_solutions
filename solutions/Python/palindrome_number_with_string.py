def is_palindrome(x: int) -> bool:
    str_x = str(x)
    return str_x == str_x[::-1]

print(is_palindrome(121))  # True
print(is_palindrome(-121)) # False
print(is_palindrome(10))   # False
