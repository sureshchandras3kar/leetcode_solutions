class Solution {
    public double findMedianSortedArrays(int[] nums1, int[] nums2) {
        if (nums1.length > nums2.length) {
            return findMedianSortedArrays(nums2, nums1);
        }

        int m = nums1.length, n = nums2.length;
        int left = 0, right = m;

        while (left <= right) {
            int partition1 = (left + right) / 2;
            int partition2 = (m + n + 1) / 2 - partition1;

            int leftMax1 = (partition1 == 0) ? Integer.MIN_VALUE : nums1[partition1 - 1];
            int rightMin1 = (partition1 == m) ? Integer.MAX_VALUE : nums1[partition1];

            int leftMax2 = (partition2 == 0) ? Integer.MIN_VALUE : nums2[partition2 - 1];
            int rightMin2 = (partition2 == n) ? Integer.MAX_VALUE : nums2[partition2];

            if (leftMax1 <= rightMin2 && leftMax2 <= rightMin1) {
                if ((m + n) % 2 == 0) {
                    return (Math.max(leftMax1, leftMax2) + Math.min(rightMin1, rightMin2)) / 2.0;
                } else {
                    return Math.max(leftMax1, leftMax2);
                }
            } else if (leftMax1 > rightMin2) {
                right = partition1 - 1;
            } else {
                left = partition1 + 1;
            }
        }

        return -1.0;
    }
}

System.out.println(new Solution().findMedianSortedArrays(new int[]{1, 3}, new int[]{2}));
