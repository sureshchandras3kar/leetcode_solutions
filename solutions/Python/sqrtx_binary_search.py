def sqrtx_binary_search(x: int) -> int:
    """Find integer square root using binary search."""
    if x < 2:
        return x

    left, right = 2, x // 2
    while left <= right:
        mid = (left + right) // 2
        if mid * mid == x:
            return mid
        elif mid * mid < x:
            left = mid + 1
        else:
            right = mid - 1

    return right

# Test cases
print(sqrtx_binary_search(4))  # 2
print(sqrtx_binary_search(8))  # 2
