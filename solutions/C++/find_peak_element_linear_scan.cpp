#include <vector>
using namespace std;

/*
#162 Find Peak Element - Linear Scan Approach
Time: O(n), Space: O(1)

Iterate through array and find the first element greater than its neighbors.
*/

class Solution {
public:
    int findPeakElement(vector<int>& nums) {
        /*
        Linear scan to find first peak.
        Compare each element with its neighbors.
        */
        for (int i = 0; i < nums.size(); i++) {
            // Check if current element is greater than neighbors
            bool is_peak = true;

            if (i > 0 && nums[i] <= nums[i - 1]) {
                is_peak = false;
            }

            if (i < nums.size() - 1 && nums[i] <= nums[i + 1]) {
                is_peak = false;
            }

            if (is_peak) {
                return i;
            }
        }

        // If no peak found (shouldn't happen given constraints)
        return 0;
    }
};
