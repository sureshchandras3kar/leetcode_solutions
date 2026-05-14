function findMedianSortedArrays(nums1, nums2) {
    if (nums1.length > nums2.length) {
        return findMedianSortedArrays(nums2, nums1);
    }

    const m = nums1.length, n = nums2.length;
    let left = 0, right = m;

    while (left <= right) {
        const partition1 = Math.floor((left + right) / 2);
        const partition2 = Math.floor((m + n + 1) / 2) - partition1;

        const left_max1 = partition1 === 0 ? -Infinity : nums1[partition1 - 1];
        const right_min1 = partition1 === m ? Infinity : nums1[partition1];
        const left_max2 = partition2 === 0 ? -Infinity : nums2[partition2 - 1];
        const right_min2 = partition2 === n ? Infinity : nums2[partition2];

        if (left_max1 <= right_min2 && left_max2 <= right_min1) {
            return (m + n) % 2 === 0 ?
                (Math.max(left_max1, left_max2) + Math.min(right_min1, right_min2)) / 2 :
                Math.max(left_max1, left_max2);
        } else if (left_max1 > right_min2) {
            right = partition1 - 1;
        } else {
            left = partition1 + 1;
        }
    }

    return -1.0;
}
