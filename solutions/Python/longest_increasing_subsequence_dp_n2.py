from typing import List


def longest_increasing_subsequence_dp(nums: List[int]) -> int:
    """
    Find length of longest increasing subsequence using DP O(n²).

    Time Complexity: O(n²)
    Space Complexity: O(n)

    dp[i] = length of LIS ending at index i
    For each i, check all j < i where nums[j] < nums[i]
    dp[i] = max(dp[j]) + 1 for all valid j, or 1 if no such j exists
    """
    if not nums:
        return 0

    n = len(nums)
    dp = [1] * n  # Each element is a LIS of length 1

    for i in range(1, n):
        for j in range(i):
            if nums[j] < nums[i]:
                dp[i] = max(dp[i], dp[j] + 1)

    return max(dp)


if __name__ == "__main__":
    print(longest_increasing_subsequence_dp([10, 9, 2, 5, 3, 7, 101, 18]))  # 4
    print(longest_increasing_subsequence_dp([0, 1, 0, 4, 4, 4, 3, 2, 1]))   # 2
    print(longest_increasing_subsequence_dp([3, 1, 4, 1, 5, 9, 2, 6]))      # 4
