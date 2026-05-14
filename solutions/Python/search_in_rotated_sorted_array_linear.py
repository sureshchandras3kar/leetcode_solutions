"""
#33 Search in Rotated Sorted Array - Linear Search Approach
Time: O(n), Space: O(1)

Linear scan through array to find target value.
"""


class Solution:
    def search(self, nums: list[int], target: int) -> int:
        """
        Simple linear search through array.
        Return index if found, otherwise -1.
        """
        for i in range(len(nums)):
            if nums[i] == target:
                return i

        return -1


# Test cases
if __name__ == "__main__":
    sol = Solution()

    assert sol.search([4, 5, 6, 7, 0, 1, 2], 0) == 4
    assert sol.search([4, 5, 6, 7, 0, 1, 2], 3) == -1
    assert sol.search([1], 1) == 0
    assert sol.search([1, 3], 3) == 1
    assert sol.search([3, 1], 3) == 0

    print("All test cases passed!")
