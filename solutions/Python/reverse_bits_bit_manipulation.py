def reverse_bits_bit_manipulation(n: int) -> int:
    """
    Bit manipulation approach - reverse bits one by one using bit operations.
    Time: O(1) - exactly 32 iterations for a 32-bit integer
    Space: O(1) - constant extra space
    """
    result = 0
    for i in range(32):
        # Extract the rightmost bit of n
        bit = n & 1
        # Shift result left and add the bit
        result = (result << 1) | bit
        # Shift n right for next iteration
        n >>= 1
    return result


print(reverse_bits_bit_manipulation(0b00000010100101000001111010011100))  # 964176192
print(reverse_bits_bit_manipulation(0b11111111111111111111111111111101)) # 3221225469
