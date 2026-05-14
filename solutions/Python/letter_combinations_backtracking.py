from typing import List


def letter_combinations_backtracking(digits: str) -> List[str]:
    """
    Generate all letter combinations using backtracking (recursive).
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

    result = []

    def backtrack(index: int, current_combination: str) -> None:
        # Base case: we've processed all digits
        if index == len(digits):
            result.append(current_combination)
            return

        # Get the letters that the current digit maps to
        current_digit = digits[index]
        letters = phone_map[current_digit]

        # Try each letter
        for letter in letters:
            backtrack(index + 1, current_combination + letter)

    backtrack(0, "")
    return result


print(letter_combinations_backtracking("23"))  # ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
print(letter_combinations_backtracking(""))  # []
print(letter_combinations_backtracking("2"))  # ["a", "b", "c"]
