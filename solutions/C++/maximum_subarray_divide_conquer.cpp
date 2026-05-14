#include <iostream>
#include <vector>
#include <algorithm>
#include <climits>

using namespace std;

/**
 * Divide and Conquer approach: Split the array, find max subarrays in each half,
 * and at the crossing point, then return the maximum.
 * Time: O(n log n), Space: O(log n) for recursion stack
 */
class MaximumSubarrayDC {
private:
    int maxCrossingSubarray(vector<int>& nums, int left, int mid, int right) {
        // Sum from mid to left
        int left_sum = INT_MIN;
        int sum = 0;
        for (int i = mid; i >= left; i--) {
            sum += nums[i];
            left_sum = max(left_sum, sum);
        }

        // Sum from mid+1 to right
        int right_sum = INT_MIN;
        sum = 0;
        for (int i = mid + 1; i <= right; i++) {
            sum += nums[i];
            right_sum = max(right_sum, sum);
        }

        return left_sum + right_sum;
    }

    int helper(vector<int>& nums, int left, int right) {
        if (left == right) {
            return nums[left];
        }

        int mid = left + (right - left) / 2;

        // Maximum in left half, right half, and crossing the middle
        int left_max = helper(nums, left, mid);
        int right_max = helper(nums, mid + 1, right);
        int cross_max = maxCrossingSubarray(nums, left, mid, right);

        return max({left_max, right_max, cross_max});
    }

public:
    int maxSubarray(vector<int>& nums) {
        return helper(nums, 0, nums.size() - 1);
    }
};

int main() {
    MaximumSubarrayDC solution;

    vector<int> nums1 = {-2, 1, -3, 4, -1, 2, 1, -5, 4};
    cout << solution.maxSubarray(nums1) << endl;  // 6

    vector<int> nums2 = {5};
    cout << solution.maxSubarray(nums2) << endl;  // 5

    vector<int> nums3 = {-5, -2, -3};
    cout << solution.maxSubarray(nums3) << endl;  // -2

    return 0;
}
