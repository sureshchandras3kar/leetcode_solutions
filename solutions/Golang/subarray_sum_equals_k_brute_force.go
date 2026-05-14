func subarraySum(nums []int, k int) int {
    result := 0
    for i := 0; i < len(nums); i++ {
        total := 0
        for j := i; j < len(nums); j++ {
            total += nums[j]
            if total == k {
                result++
            }
        }
    }
    return result
}
