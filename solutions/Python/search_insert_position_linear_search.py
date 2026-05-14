"""
#35 Search Insert Position - Linear Search Approach
Time: O(n), Space: O(1)

Iterate through array to find position or insertion point.
"""


class Solution:
    def searchInsert(self, nums: list[int], target: int) -> int:
        """
        Linear search through array.
        Return index when found or where it should be inserted.
        """
        for i in range(len(nums)):
            if nums[i] >= target:
                return i

        # If target larger than all elements
        return len(nums)


# Test cases
if __name__ == "__main__":
    sol = Solution()

    assert sol.searchInsert([1, 3, 5, 6], 5) == 2
    assert sol.searchInsert([1, 3, 5, 6], 2) == 1
    assert sol.searchInsert([1, 3, 5, 6], 7) == 4
    assert sol.searchInsert([1, 3, 5, 6], 0) == 0
    assert sol.searchInsert([], 1) == 0

    print("All test cases passed!")
