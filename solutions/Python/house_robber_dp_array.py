from typing import List


def house_robber_dp_array(nums: List[int]) -> int:
    """
    Rob houses with maximum money using DP array approach.

    Time Complexity: O(n)
    Space Complexity: O(n)

    dp[i] = maximum money robbing houses 0 to i
    For each house i, we choose the max of:
    - Rob house i + dp[i-2] (skip i-1)
    - Don't rob house i + dp[i-1] (keep i-1)
    """
    if not nums:
        return 0
    if len(nums) == 1:
        return nums[0]

    dp = [0] * len(nums)
    dp[0] = nums[0]
    dp[1] = max(nums[0], nums[1])

    for i in range(2, len(nums)):
        dp[i] = max(nums[i] + dp[i - 2], dp[i - 1])

    return dp[-1]


if __name__ == "__main__":
    print(house_robber_dp_array([1, 2, 3, 1]))      # 4 (rob house 0 and 2)
    print(house_robber_dp_array([2, 7, 9, 3, 1]))   # 12 (rob houses 1 and 3)
    print(house_robber_dp_array([5, 3, 4, 11, 2]))  # 16 (rob houses 0, 2, 4)
