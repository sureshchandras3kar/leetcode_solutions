#include <vector>
#include <algorithm>
using namespace std;

double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
    if (nums1.size() > nums2.size()) {
        return findMedianSortedArrays(nums2, nums1);
    }

    int m = nums1.size(), n = nums2.size();
    int left = 0, right = m;

    while (left <= right) {
        int partition1 = (left + right) / 2;
        int partition2 = (m + n + 1) / 2 - partition1;

        int left_max1 = (partition1 == 0) ? INT_MIN : nums1[partition1 - 1];
        int right_min1 = (partition1 == m) ? INT_MAX : nums1[partition1];
        int left_max2 = (partition2 == 0) ? INT_MIN : nums2[partition2 - 1];
        int right_min2 = (partition2 == n) ? INT_MAX : nums2[partition2];

        if (left_max1 <= right_min2 && left_max2 <= right_min1) {
            if ((m + n) % 2 == 0) {
                return (max(left_max1, left_max2) + min(right_min1, right_min2)) / 2.0;
            } else {
                return max(left_max1, left_max2);
            }
        } else if (left_max1 > right_min2) {
            right = partition1 - 1;
        } else {
            left = partition1 + 1;
        }
    }

    return -1.0;
}
