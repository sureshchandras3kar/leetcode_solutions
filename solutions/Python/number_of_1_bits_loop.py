def number_of_1_bits_loop(n: int) -> int:
    """Count 1 bits by checking each bit."""
    count = 0
    while n:
        count += n & 1
        n >>= 1
    return count

# Test cases
print(number_of_1_bits_loop(11))  # 3
print(number_of_1_bits_loop(128))  # 1
