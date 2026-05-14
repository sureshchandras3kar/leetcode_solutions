from typing import List

def generateParenthesis(n: int) -> List[str]:
    """Backtracking approach: O(4^n) time, O(n) space"""
    result = []

    def backtrack(current, left, right):
        if len(current) == 2 * n:
            result.append(current)
            return
        if left < n:
            backtrack(current + "(", left + 1, right)
        if right < left:
            backtrack(current + ")", left, right + 1)

    backtrack("", 0, 0)
    return result
