/*
#35 Search Insert Position - Linear Search Approach
Time: O(n), Space: O(1)

Iterate through array to find position or insertion point.
*/

class Solution {
    public int searchInsert(int[] nums, int target) {
        /*
        Linear search through array.
        Return index when found or where it should be inserted.
        */
        for (int i = 0; i < nums.length; i++) {
            if (nums[i] >= target) {
                return i;
            }
        }

        // If target larger than all elements
        return nums.length;
    }
}
