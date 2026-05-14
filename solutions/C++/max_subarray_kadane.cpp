#include <vector>
#include <algorithm>

using namespace std;

int maxSubArray(vector<int>& nums) {
    /**
     * Kadane's Algorithm - Find the maximum sum of any contiguous subarray.
     *
     * Time Complexity: O(n) - Single pass through the array
     * Space Complexity: O(1) - Only tracking current and max sums
     *
     * @param nums Vector of integers
     * @return Maximum subarray sum
     */
    int max_sum = nums[0];
    int current_sum = nums[0];

    for (int i = 1; i < nums.size(); i++) {
        // Decide whether to extend or start fresh
        current_sum = max(nums[i], current_sum + nums[i]);
        // Update overall maximum
        max_sum = max(max_sum, current_sum);
    }

    return max_sum;
}
