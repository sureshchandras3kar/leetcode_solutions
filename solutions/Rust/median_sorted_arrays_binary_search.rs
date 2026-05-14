pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (nums1, nums2) = if nums1.len() <= nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };

    let m = nums1.len();
    let n = nums2.len();
    let mut left = 0;
    let mut right = m;

    loop {
        let partition1 = (left + right) / 2;
        let partition2 = (m + n + 1) / 2 - partition1;

        let left_max1 = if partition1 == 0 { i32::MIN } else { nums1[partition1 - 1] };
        let right_min1 = if partition1 == m { i32::MAX } else { nums1[partition1] };
        let left_max2 = if partition2 == 0 { i32::MIN } else { nums2[partition2 - 1] };
        let right_min2 = if partition2 == n { i32::MAX } else { nums2[partition2] };

        if left_max1 <= right_min2 && left_max2 <= right_min1 {
            return if (m + n) % 2 == 0 {
                (left_max1.max(left_max2) as f64 + right_min1.min(right_min2) as f64) / 2.0
            } else {
                left_max1.max(left_max2) as f64
            };
        } else if left_max1 > right_min2 {
            right = partition1 - 1;
        } else {
            left = partition1 + 1;
        }
    }
}
