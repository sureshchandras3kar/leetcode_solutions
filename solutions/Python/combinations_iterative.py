from typing import List


def combinations_iterative(n: int, k: int) -> List[List[int]]:
    """
    Generate all combinations of k numbers from 1 to n using iterative approach.
    Time: O(C(n,k) * k), Space: O(C(n,k) * k) for result
    """
    result = []
    combo = list(range(1, k + 1))

    while True:
        result.append(combo[:])

        # Find the rightmost number that can be incremented
        i = k - 1
        while i >= 0 and combo[i] == n - k + i + 1:
            i -= 1

        if i < 0:
            break

        # Increment and reset
        combo[i] += 1
        for j in range(i + 1, k):
            combo[j] = combo[j - 1] + 1

    return result


print(combinations_iterative(4, 2))  # [[1,2], [1,3], [1,4], [2,3], [2,4], [3,4]]
print(combinations_iterative(1, 1))  # [[1]]
print(combinations_iterative(5, 3))  # 10 combinations
