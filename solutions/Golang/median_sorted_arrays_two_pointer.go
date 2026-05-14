package main

func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
    merged := []int{}
    i, j := 0, 0

    for i < len(nums1) && j < len(nums2) {
        if nums1[i] <= nums2[j] {
            merged = append(merged, nums1[i])
            i++
        } else {
            merged = append(merged, nums2[j])
            j++
        }
    }

    for i < len(nums1) {
        merged = append(merged, nums1[i])
        i++
    }
    for j < len(nums2) {
        merged = append(merged, nums2[j])
        j++
    }

    n := len(merged)
    if n%2 == 1 {
        return float64(merged[n/2])
    }
    return float64(merged[n/2-1]+merged[n/2]) / 2.0
}
