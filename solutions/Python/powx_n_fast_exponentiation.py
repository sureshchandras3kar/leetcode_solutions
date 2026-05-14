def powx_n_fast_exponentiation(x: float, n: int) -> float:
    """Calculate x^n using fast exponentiation."""
    if n == 0:
        return 1.0
    if n < 0:
        x = 1 / x
        n = -n

    result = 1.0
    while n > 0:
        if n % 2 == 1:
            result *= x
        x *= x
        n //= 2

    return result

# Test cases
print(powx_n_fast_exponentiation(2.0, 10))  # 1024.0
print(powx_n_fast_exponentiation(2.1, 3))  # 9.261
