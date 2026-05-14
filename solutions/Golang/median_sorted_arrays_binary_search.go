package main

import "math"

func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
    if len(nums1) > len(nums2) {
        return findMedianSortedArrays(nums2, nums1)
    }

    m, n := len(nums1), len(nums2)
    left, right := 0, m

    for left <= right {
        partition1 := (left + right) / 2
        partition2 := (m + n + 1) / 2 - partition1

        left_max1 := math.MinInt32
        if partition1 != 0 { left_max1 = nums1[partition1-1] }
        right_min1 := math.MaxInt32
        if partition1 != m { right_min1 = nums1[partition1] }

        left_max2 := math.MinInt32
        if partition2 != 0 { left_max2 = nums2[partition2-1] }
        right_min2 := math.MaxInt32
        if partition2 != n { right_min2 = nums2[partition2] }

        if left_max1 <= right_min2 && left_max2 <= right_min1 {
            if (m+n)%2 == 0 {
                return float64(max(left_max1, left_max2)+min(right_min1, right_min2)) / 2.0
            } else {
                return float64(max(left_max1, left_max2))
            }
        } else if left_max1 > right_min2 {
            right = partition1 - 1
        } else {
            left = partition1 + 1
        }
    }

    return -1.0
}

func max(a, b int) int { if a > b { return a }; return b }
func min(a, b int) int { if a < b { return a }; return b }
