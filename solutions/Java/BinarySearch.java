public class BinarySearch {
    public double findMedianSortedArrays(int[] nums1, int[] nums2) {
        if (nums1.length > nums2.length) {
            return findMedianSortedArrays(nums2, nums1);
        }

        int m = nums1.length, n = nums2.length;
        int left = 0, right = m;

        while (left <= right) {
            int partition1 = (left + right) / 2;
            int partition2 = (m + n + 1) / 2 - partition1;

            int left_max1 = partition1 == 0 ? Integer.MIN_VALUE : nums1[partition1 - 1];
            int right_min1 = partition1 == m ? Integer.MAX_VALUE : nums1[partition1];
            int left_max2 = partition2 == 0 ? Integer.MIN_VALUE : nums2[partition2 - 1];
            int right_min2 = partition2 == n ? Integer.MAX_VALUE : nums2[partition2];

            if (left_max1 <= right_min2 && left_max2 <= right_min1) {
                return (m + n) % 2 == 0 ?
                    (Math.max(left_max1, left_max2) + Math.min(right_min1, right_min2)) / 2.0 :
                    Math.max(left_max1, left_max2);
            } else if (left_max1 > right_min2) {
                right = partition1 - 1;
            } else {
                left = partition1 + 1;
            }
        }

        return -1.0;
    }
}
