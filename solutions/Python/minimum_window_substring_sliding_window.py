from typing import Dict


def min_window_sliding_window(s: str, t: str) -> str:
    if not s or not t or len(s) < len(t):
        return ""

    # Dictionary to store frequency of characters in t
    dict_t: Dict[str, int] = {}
    for char in t:
        dict_t[char] = dict_t.get(char, 0) + 1

    required = len(dict_t)
    formed = 0

    window_counts: Dict[str, int] = {}

    # ans tuple of the form (window length, left, right)
    ans = float("inf"), None, None

    l = 0

    for r in range(len(s)):
        # Add one character from the right to the window
        char = s[r]
        window_counts[char] = window_counts.get(char, 0) + 1

        # If frequency of current character added equals the desired count in t
        # then increment the formed count
        if char in dict_t and window_counts[char] == dict_t[char]:
            formed += 1

        # Try to contract the window until the point where it ceases to be 'desirable'
        while l <= r and formed == required:
            char = s[l]

            # Save the smallest window
            if r - l + 1 < ans[0]:
                ans = (r - l + 1, l, r)

            # The character at the position pointed by the `left` pointer is no longer
            # a part of the window
            window_counts[char] -= 1
            if char in dict_t and window_counts[char] < dict_t[char]:
                formed -= 1

            # Move the left pointer ahead for the next iteration
            l += 1

    # Return the smallest window or empty string
    return "" if ans[0] == float("inf") else s[ans[1] : ans[2] + 1]


print(min_window_sliding_window("ADOBECODEBANC", "ABC"))  # "BANC"
print(min_window_sliding_window("a", "a"))  # "a"
print(min_window_sliding_window("a", "aa"))  # ""
