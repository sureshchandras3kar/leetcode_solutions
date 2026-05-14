def longest_substring_without_repeating_characters_sliding_window(s: str) -> int:
    char_index = {}
    max_length = 0
    left = 0

    for right in range(len(s)):
        if s[right] in char_index and char_index[s[right]] >= left:
            left = char_index[s[right]] + 1
        char_index[s[right]] = right
        max_length = max(max_length, right - left + 1)

    return max_length

print(longest_substring_without_repeating_characters_sliding_window("abcabcbb"))  # 3
