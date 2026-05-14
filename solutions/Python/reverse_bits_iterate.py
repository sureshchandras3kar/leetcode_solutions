def reverse_bits_iterate(n: int) -> int:
    """Reverse bits of a 32-bit unsigned integer by iteration."""
    result = 0
    for i in range(32):
        result = (result << 1) | (n & 1)
        n >>= 1
    return result

# Test cases
print(reverse_bits_iterate(43261596))  # 964176192
