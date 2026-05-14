#include <vector>
#include <algorithm>

using namespace std;

int maxSubArray(vector<int>& nums) {
    /**
     * Dynamic Programming approach - Build up subarray sums.
     *
     * Time Complexity: O(n)
     * Space Complexity: O(1) - Optimized space version
     *
     * @param nums Vector of integers
     * @return Maximum subarray sum
     */
    int n = nums.size();
    if (n == 1) return nums[0];

    int max_sum = nums[0];
    int dp = nums[0];  // max sum ending at current position

    for (int i = 1; i < n; i++) {
        dp = max(nums[i], dp + nums[i]);
        max_sum = max(max_sum, dp);
    }

    return max_sum;
}
