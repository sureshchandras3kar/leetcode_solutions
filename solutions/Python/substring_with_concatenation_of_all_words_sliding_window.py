from typing import List
from collections import defaultdict


def find_substring_sliding_window(s: str, words: List[str]) -> List[int]:
    """
    Find all starting indices of substrings that are concatenations of all words.

    Uses sliding window with word frequency tracking.
    Time: O(n * m), where n = len(s), m = len(words)
    Space: O(m * k), where k = average word length
    """
    if not words or not s:
        return []

    word_len = len(words[0])
    word_count = len(words)
    total_len = word_len * word_count

    # Count frequency of each word
    word_freq = defaultdict(int)
    for word in words:
        word_freq[word] += 1

    result = []

    # For each possible starting position
    for i in range(len(s) - total_len + 1):
        # Check if substring starting at i is a concatenation
        window_freq = defaultdict(int)

        # Extract and count words in this window
        for j in range(word_count):
            word = s[i + j * word_len:i + (j + 1) * word_len]
            window_freq[word] += 1

        # Check if frequencies match
        if window_freq == word_freq:
            result.append(i)

    return result


# Test cases
if __name__ == "__main__":
    # Example 1
    s1 = "barfoothefoobarman"
    words1 = ["foo", "bar"]
    print(find_substring_sliding_window(s1, words1))  # [0, 9]

    # Example 2
    s2 = "wordgoodgoodgoodbestword"
    words2 = ["word", "good", "best", "word"]
    print(find_substring_sliding_window(s2, words2))  # []

    # Example 3
    s3 = "barfoofoobarthefoobarman"
    words3 = ["bar", "foo", "the"]
    print(find_substring_sliding_window(s3, words3))  # [6, 9, 12]
