"""
#35 Search Insert Position - Binary Search Approach
Time: O(log n), Space: O(1)

Given a sorted array and a target value, return the index if found,
otherwise return the index where it would be if it were inserted.
"""


class Solution:
    def searchInsert(self, nums: list[int], target: int) -> int:
        """
        Binary search to find insert position.
        Left pointer converges to insertion point.
        """
        left, right = 0, len(nums) - 1

        while left <= right:
            mid = left + (right - left) // 2

            if nums[mid] == target:
                return mid
            elif nums[mid] < target:
                left = mid + 1
            else:
                right = mid - 1

        # left is at insertion position when target not found
        return left


# Test cases
if __name__ == "__main__":
    sol = Solution()

    assert sol.searchInsert([1, 3, 5, 6], 5) == 2
    assert sol.searchInsert([1, 3, 5, 6], 2) == 1
    assert sol.searchInsert([1, 3, 5, 6], 7) == 4
    assert sol.searchInsert([1, 3, 5, 6], 0) == 0
    assert sol.searchInsert([], 1) == 0

    print("All test cases passed!")
