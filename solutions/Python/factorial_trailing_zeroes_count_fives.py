def factorial_trailing_zeroes_count_fives(n: int) -> int:
    """Count trailing zeroes in n! by counting factors of 5."""
    count = 0
    power_of_5 = 5
    while power_of_5 <= n:
        count += n // power_of_5
        power_of_5 *= 5
    return count

# Test cases
print(factorial_trailing_zeroes_count_fives(5))  # 1
print(factorial_trailing_zeroes_count_fives(25))  # 6
