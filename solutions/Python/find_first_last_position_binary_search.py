"""
#34 Find First and Last Position of Element in Sorted Array - Binary Search
Time: O(log n), Space: O(1)

Find the starting and ending position of a given target value in a sorted array.
Use two binary searches: one to find first position, one to find last.
"""


class Solution:
    def searchRange(self, nums: list[int], target: int) -> list[int]:
        """
        Two binary searches to find first and last positions.
        """
        if not nums:
            return [-1, -1]

        # Find first position
        left, right = 0, len(nums) - 1
        first_pos = -1

        while left <= right:
            mid = left + (right - left) // 2
            if nums[mid] == target:
                first_pos = mid
                right = mid - 1  # Continue searching left
            elif nums[mid] < target:
                left = mid + 1
            else:
                right = mid - 1

        # If target not found
        if first_pos == -1:
            return [-1, -1]

        # Find last position
        left, right = 0, len(nums) - 1
        last_pos = -1

        while left <= right:
            mid = left + (right - left) // 2
            if nums[mid] == target:
                last_pos = mid
                left = mid + 1  # Continue searching right
            elif nums[mid] < target:
                left = mid + 1
            else:
                right = mid - 1

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
