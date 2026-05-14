#include <vector>
using namespace std;

/*
#33 Search in Rotated Sorted Array - Binary Search Approach
Time: O(log n), Space: O(1)

A rotated sorted array has been rotated at an unknown pivot.
Use binary search to find target, identifying which half is sorted.
*/

class Solution {
public:
    int search(vector<int>& nums, int target) {
        /*
        Binary search on rotated array.
        Determine which half is sorted, then check if target is in that half.
        */
        int left = 0, right = nums.size() - 1;

        while (left <= right) {
            int mid = left + (right - left) / 2;

            if (nums[mid] == target) {
                return mid;
            }

            // Determine which half is sorted
            if (nums[left] <= nums[mid]) {
                // Left half is sorted
                if (nums[left] <= target && target < nums[mid]) {
                    // Target is in sorted left half
                    right = mid - 1;
                } else {
                    // Target is in right half
                    left = mid + 1;
                }
            } else {
                // Right half is sorted
                if (nums[mid] < target && target <= nums[right]) {
                    // Target is in sorted right half
                    left = mid + 1;
                } else {
                    // Target is in left half
                    right = mid - 1;
                }
            }
        }

        return -1;
    }
};
