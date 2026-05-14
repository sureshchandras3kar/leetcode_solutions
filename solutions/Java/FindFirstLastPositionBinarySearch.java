/*
#34 Find First and Last Position of Element in Sorted Array - Binary Search
Time: O(log n), Space: O(1)

Find the starting and ending position of a given target value in a sorted array.
Use two binary searches: one to find first position, one to find last.
*/

class Solution {
    public int[] searchRange(int[] nums, int target) {
        /*
        Two binary searches to find first and last positions.
        */
        if (nums.length == 0) {
            return new int[]{-1, -1};
        }

        // Find first position
        int left = 0, right = nums.length - 1;
        int firstPos = -1;

        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (nums[mid] == target) {
                firstPos = mid;
                right = mid - 1;  // Continue searching left
            } else if (nums[mid] < target) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        // If target not found
        if (firstPos == -1) {
            return new int[]{-1, -1};
        }

        // Find last position
        left = 0;
        right = nums.length - 1;
        int lastPos = -1;

        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (nums[mid] == target) {
                lastPos = mid;
                left = mid + 1;  // Continue searching right
            } else if (nums[mid] < target) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        return new int[]{firstPos, lastPos};
    }
}
