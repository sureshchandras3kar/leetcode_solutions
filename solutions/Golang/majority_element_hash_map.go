func majorityElement(nums []int) int {
    counts := map[int]int{}
    threshold := len(nums) / 2
    for _, num := range nums {
        counts[num]++
        if counts[num] > threshold {
            return num
        }
    }
    return nums[0]
}
