

class Solution:
    """
    Palindrome Number

    a number that reads the same backward as forward. For example, 121 is palindrome.

    Given an integer x, return true if x is a
    palindrome, and false otherwise.

    Example 1:

    Input: x = 121
    Output: true
    Explanation: 121 reads as 121 from left to right and from right to left.
    """

    @staticmethod
    def is_palindrome(x: int) -> bool:
        """
        Check if x is a palindrome number and return true if x is a palindrome number and false otherwise return false
        :param x: a number to be checked against the palindrome number
        :return: true if x is a palindrome number and false otherwise return
        """
        if x < 0:
            return False
        if x == 0:
            return True
        if x % 10 == 0:
            return False
        num_reversed = 0
        source_copy = x
        while x > 0:
            last_digit = x % 10
            num_reversed = num_reversed * 10 + last_digit
            x = x // 10
        return source_copy == num_reversed
