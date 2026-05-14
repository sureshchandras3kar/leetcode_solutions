from typing import List

def plus_one_iterate(digits: List[int]) -> List[int]:
    """Increment number by one using iteration."""
    carry = 1
    for i in range(len(digits) - 1, -1, -1):
        digits[i] += carry
        if digits[i] < 10:
            return digits
        digits[i] = 0

    return [1] + digits

# Test cases
print(plus_one_iterate([1, 2, 3]))  # [1, 2, 4]
print(plus_one_iterate([9, 9, 9]))  # [1, 0, 0, 0]
