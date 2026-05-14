func intersection(nums1 []int, nums2 []int) []int {
    seen := map[int]bool{}
    result := map[int]bool{}
    for _, num := range nums1 {
        seen[num] = true
    }
    for _, num := range nums2 {
        if seen[num] {
            result[num] = true
        }
    }
    output := make([]int, 0, len(result))
    for num := range result {
        output = append(output, num)
    }
    return output
}
