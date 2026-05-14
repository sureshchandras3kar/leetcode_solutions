from typing import Dict


def min_window_brute_force(s: str, t: str) -> str:
    if not s or not t or len(s) < len(t):
        return ""

    dict_t: Dict[str, int] = {}
    for char in t:
        dict_t[char] = dict_t.get(char, 0) + 1

    min_len = float("inf")
    min_start = 0

    # Check all possible substrings
    for i in range(len(s)):
        for j in range(i + 1, len(s) + 1):
            substring = s[i:j]

            # Check if substring contains all characters from t with required frequencies
            substring_count: Dict[str, int] = {}
            for char in substring:
                substring_count[char] = substring_count.get(char, 0) + 1

            # Verify if this substring is valid
            valid = True
            for char in dict_t:
                if substring_count.get(char, 0) < dict_t[char]:
                    valid = False
                    break

            # Update minimum if this is a valid and shorter substring
            if valid and len(substring) < min_len:
                min_len = len(substring)
                min_start = i

    return s[min_start : min_start + min_len] if min_len != float("inf") else ""


print(min_window_brute_force("ADOBECODEBANC", "ABC"))  # "BANC"
print(min_window_brute_force("a", "a"))  # "a"
print(min_window_brute_force("a", "aa"))  # ""
