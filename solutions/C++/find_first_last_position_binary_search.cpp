#include <vector>
using namespace std;

/*
#34 Find First and Last Position of Element in Sorted Array - Binary Search
Time: O(log n), Space: O(1)

Find the starting and ending position of a given target value in a sorted array.
Use two binary searches: one to find first position, one to find last.
*/

class Solution {
public:
    vector<int> searchRange(vector<int>& nums, int target) {
        /*
        Two binary searches to find first and last positions.
        */
        if (nums.empty()) {
            return {-1, -1};
        }

        // Find first position
        int left = 0, right = nums.size() - 1;
        int first_pos = -1;

        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (nums[mid] == target) {
                first_pos = mid;
                right = mid - 1;  // Continue searching left
            } else if (nums[mid] < target) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        // If target not found
        if (first_pos == -1) {
            return {-1, -1};
        }

        // Find last position
        left = 0;
        right = nums.size() - 1;
        int last_pos = -1;

        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (nums[mid] == target) {
                last_pos = mid;
                left = mid + 1;  // Continue searching right
            } else if (nums[mid] < target) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        return {first_pos, last_pos};
    }
};
