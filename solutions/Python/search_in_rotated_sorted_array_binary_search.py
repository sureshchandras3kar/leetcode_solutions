"""
#33 Search in Rotated Sorted Array - Binary Search Approach
Time: O(log n), Space: O(1)

A rotated sorted array has been rotated at an unknown pivot.
Use binary search to find target, identifying which half is sorted.
"""


class Solution:
    def search(self, nums: list[int], target: int) -> int:
        """
        Binary search on rotated array.
        Determine which half is sorted, then check if target is in that half.
        """
        left, right = 0, len(nums) - 1

        while left <= right:
            mid = left + (right - left) // 2

            if nums[mid] == target:
                return mid

            # Determine which half is sorted
            if nums[left] <= nums[mid]:
                # Left half is sorted
                if nums[left] <= target < nums[mid]:
                    # Target is in sorted left half
                    right = mid - 1
                else:
                    # Target is in right half
                    left = mid + 1
            else:
                # Right half is sorted
                if nums[mid] < target <= nums[right]:
                    # Target is in sorted right half
                    left = mid + 1
                else:
                    # Target is in left half
                    right = mid - 1

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
