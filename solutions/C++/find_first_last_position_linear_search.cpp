#include <vector>
using namespace std;

/*
#34 Find First and Last Position of Element in Sorted Array - Linear Search
Time: O(n), Space: O(1)

Linear scan to find first and last occurrences of target value.
*/

class Solution {
public:
    vector<int> searchRange(vector<int>& nums, int target) {
        /*
        Linear search to find first and last positions.
        */
        if (nums.empty()) {
            return {-1, -1};
        }

        int first_pos = -1;
        int last_pos = -1;

        // Find first position
        for (int i = 0; i < nums.size(); i++) {
            if (nums[i] == target) {
                first_pos = i;
                break;
            }
        }

        // If target not found
        if (first_pos == -1) {
            return {-1, -1};
        }

        // Find last position
        for (int i = nums.size() - 1; i >= 0; i--) {
            if (nums[i] == target) {
                last_pos = i;
                break;
            }
        }

        return {first_pos, last_pos};
    }
};
