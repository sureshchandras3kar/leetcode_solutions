"""
#162 Find Peak Element - Linear Scan Approach
Time: O(n), Space: O(1)

Iterate through array and find the first element greater than its neighbors.
"""


class Solution:
    def findPeakElement(self, nums: list[int]) -> int:
        """
        Linear scan to find first peak.
        Compare each element with its neighbors.
        """
        for i in range(len(nums)):
            # Check if current element is greater than neighbors
            is_peak = True

            if i > 0 and nums[i] <= nums[i - 1]:
                is_peak = False

            if i < len(nums) - 1 and nums[i] <= nums[i + 1]:
                is_peak = False

            if is_peak:
                return i

        # If no peak found (shouldn't happen given constraints)
        return 0


# Test cases
if __name__ == "__main__":
    sol = Solution()

    assert sol.findPeakElement([1, 2, 3, 1]) == 2
    assert sol.findPeakElement([1, 2, 1]) == 1
    assert sol.findPeakElement([1]) == 0
    assert sol.findPeakElement([6, 5, 4, 3, 2, 3, 4]) == 0
    assert sol.findPeakElement([1, 2, 3, 4, 5]) == 4

    print("All test cases passed!")
