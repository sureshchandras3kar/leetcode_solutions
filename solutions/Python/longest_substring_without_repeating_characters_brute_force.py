def longest_substring_without_repeating_characters_brute_force(s: str) -> int:
    max_length = 0

    for i in range(len(s)):
        char_set = set()
        for j in range(i, len(s)):
            if s[j] in char_set:
                break
            char_set.add(s[j])
            max_length = max(max_length, j - i + 1)

    return max_length

print(longest_substring_without_repeating_characters_brute_force("abcabcbb"))  # 3
