def add_binary_bit_operations(a: str, b: str) -> str:
    """
    Bit operations approach - convert to int, add, convert back to binary.
    Time: O(max(len(a), len(b)))
    Space: O(max(len(a), len(b))) for result
    """
    num_a = int(a, 2)  # Convert binary string to integer
    num_b = int(b, 2)
    total = num_a + num_b
    return bin(total)[2:]  # Convert back to binary string (remove '0b' prefix)


print(add_binary_bit_operations("11", "1"))      # "100"
print(add_binary_bit_operations("1010", "1011")) # "10101"
print(add_binary_bit_operations("0", "0"))       # "0"
