from typing import List

def generateParenthesis(n: int) -> List[str]:
    """DP approach: O(4^n) time, O(n) space"""
    if n == 0:
        return [""]
    dp = [[] for _ in range(n + 1)]
    dp[0] = [""]

    for i in range(1, n + 1):
        for j in range(i):
            for left in dp[j]:
                for right in dp[i - 1 - j]:
                    dp[i].append("(" + left + ")" + right)

    return dp[n]
