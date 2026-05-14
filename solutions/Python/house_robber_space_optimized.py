from typing import List


def house_robber_space_optimized(nums: List[int]) -> int:
    """
    Rob houses with maximum money using space-optimized approach.

    Time Complexity: O(n)
    Space Complexity: O(1)

    We only need the previous two values: prev2 (dp[i-2]) and prev1 (dp[i-1])
    To rob house i optimally, we track:
    - prev2: max money up to house i-2
    - prev1: max money up to house i-1
    """
    if not nums:
        return 0
    if len(nums) == 1:
        return nums[0]

    prev2 = nums[0]          # dp[0]
    prev1 = max(nums[0], nums[1])  # dp[1]

    for i in range(2, len(nums)):
        current = max(nums[i] + prev2, prev1)
        prev2 = prev1
        prev1 = current

    return prev1


if __name__ == "__main__":
    print(house_robber_space_optimized([1, 2, 3, 1]))      # 4
    print(house_robber_space_optimized([2, 7, 9, 3, 1]))   # 12
    print(house_robber_space_optimized([5, 3, 4, 11, 2]))  # 16
