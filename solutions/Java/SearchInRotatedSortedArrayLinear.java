/*
#33 Search in Rotated Sorted Array - Linear Search Approach
Time: O(n), Space: O(1)

Linear scan through array to find target value.
*/

class Solution {
    public int search(int[] nums, int target) {
        /*
        Simple linear search through array.
        Return index if found, otherwise -1.
        */
        for (int i = 0; i < nums.length; i++) {
            if (nums[i] == target) {
                return i;
            }
        }

        return -1;
    }
}
