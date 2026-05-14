from typing import List
from collections import deque


def letter_combinations_bfs(digits: str) -> List[str]:
    """
    Generate all letter combinations using BFS (iterative).
    Time: O(4^n), Space: O(4^n) for result
    """
    if not digits:
        return []

    phone_map = {
        "2": "abc",
        "3": "def",
        "4": "ghi",
        "5": "jkl",
        "6": "mno",
        "7": "pqrs",
        "8": "tuv",
        "9": "wxyz",
    }

    # Start with an empty combination
    queue = deque([""])

    for digit in digits:
        size = len(queue)
        letters = phone_map[digit]

        # Process all current combinations
        for _ in range(size):
            current = queue.popleft()
            # Create a new combination for each letter
            for letter in letters:
                queue.append(current + letter)

    return list(queue)


print(letter_combinations_bfs("23"))  # ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
print(letter_combinations_bfs(""))  # []
print(letter_combinations_bfs("2"))  # ["a", "b", "c"]
