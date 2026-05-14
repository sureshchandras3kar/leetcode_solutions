from typing import List
from collections import Counter
from itertools import permutations


def find_substring_brute_force(s: str, words: List[str]) -> List[int]:
    """
    Find all starting indices of substrings that are concatenations of all words.

    Uses brute force by checking every possible concatenation.
    Time: O(n² * m), where n = len(s), m = len(words)
    Space: O(m * k), where k = average word length
    """
    if not words or not s:
        return []

    word_len = len(words[0])
    word_count = len(words)
    total_len = word_len * word_count

    result = []

    # For each possible starting position in the string
    for i in range(len(s) - total_len + 1):
        # Extract the substring of exact length
        substring = s[i:i + total_len]

        # Check if this substring can be formed by concatenating all words
        temp_words = words[:]
        valid = True

        for j in range(word_count):
            word = substring[j * word_len:(j + 1) * word_len]
            if word in temp_words:
                temp_words.remove(word)
            else:
                valid = False
                break

        if valid and not temp_words:
            result.append(i)

    return result


# Test cases
if __name__ == "__main__":
    # Example 1
    s1 = "barfoothefoobarman"
    words1 = ["foo", "bar"]
    print(find_substring_brute_force(s1, words1))  # [0, 9]

    # Example 2
    s2 = "wordgoodgoodgoodbestword"
    words2 = ["word", "good", "best", "word"]
    print(find_substring_brute_force(s2, words2))  # []

    # Example 3
    s3 = "barfoofoobarthefoobarman"
    words3 = ["bar", "foo", "the"]
    print(find_substring_brute_force(s3, words3))  # [6, 9, 12]
