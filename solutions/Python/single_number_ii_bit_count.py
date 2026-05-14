from typing import List

def single_number_ii_bit_count(nums: List[int]) -> int:
    """Find single number when others appear three times using bit counting."""
    bit_counts = [0] * 32
    for num in nums:
        for i in range(32):
            if num & (1 << i):
                bit_counts[i] += 1

    result = 0
    for i in range(32):
        if bit_counts[i] % 3:
            result |= (1 << i)

    # Handle negative numbers (two's complement)
    if result >= 2**31:
        result -= 2**32
    return result

# Test cases
print(single_number_ii_bit_count([2, 2, 3, 2]))  # 3
