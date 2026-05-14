"""
#34 Find First and Last Position of Element in Sorted Array - Linear Search
Time: O(n), Space: O(1)

Linear scan to find first and last occurrences of target value.
"""


class Solution:
    def searchRange(self, nums: list[int], target: int) -> list[int]:
        """
        Linear search to find first and last positions.
        """
        if not nums:
            return [-1, -1]

        first_pos = -1
        last_pos = -1

        # Find first position
        for i in range(len(nums)):
            if nums[i] == target:
                first_pos = i
                break

        # If target not found
        if first_pos == -1:
            return [-1, -1]

        # Find last position
        for i in range(len(nums) - 1, -1, -1):
            if nums[i] == target:
                last_pos = i
                break

        return [first_pos, last_pos]


# Test cases
if __name__ == "__main__":
    sol = Solution()

    assert sol.searchRange([5, 7, 7, 8, 8, 10], 8) == [3, 4]
    assert sol.searchRange([5, 7, 7, 8, 8, 10], 6) == [-1, -1]
    assert sol.searchRange([], 0) == [-1, -1]
    assert sol.searchRange([1], 1) == [0, 0]
    assert sol.searchRange([1, 3], 3) == [1, 1]

    print("All test cases passed!")
