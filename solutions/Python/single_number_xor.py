from typing import List

def single_number_xor(nums: List[int]) -> int:
    """Find single number using XOR operation."""
    result = 0
    for num in nums:
        result ^= num
    return result

# Test cases
print(single_number_xor([2, 2, 1]))  # 1
print(single_number_xor([4, 1, 2, 1, 2]))  # 4
