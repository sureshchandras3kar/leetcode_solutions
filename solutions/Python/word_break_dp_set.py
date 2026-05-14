from typing import List


def word_break_dp_set(s: str, word_dict: List[str]) -> bool:
    """
    Determine if string can be segmented using words from dictionary.

    Time Complexity: O(n²) where n = len(s)
    Space Complexity: O(n) for DP array + O(m) for set where m = len(word_dict)

    dp[i] = True if s[0:i] can be segmented
    For each position i, check all previous positions j where dp[j] = True
    and s[j:i] is in the word dictionary.
    """
    word_set = set(word_dict)
    dp = [False] * (len(s) + 1)
    dp[0] = True  # Empty string can be segmented

    for i in range(1, len(s) + 1):
        for j in range(i):
            if dp[j] and s[j:i] in word_set:
                dp[i] = True
                break

    return dp[len(s)]


if __name__ == "__main__":
    print(word_break_dp_set("leetcode", ["leet", "code"]))           # True
    print(word_break_dp_set("applepenapple", ["apple", "pen"]))      # True
    print(word_break_dp_set("catsandog", ["cat", "cats", "and", "sand", "dog"]))  # False
