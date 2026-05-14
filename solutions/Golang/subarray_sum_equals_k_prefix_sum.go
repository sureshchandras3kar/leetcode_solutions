func subarraySum(nums []int, k int) int {
    count := map[int]int{0: 1}
    prefix, result := 0, 0
    for _, num := range nums {
        prefix += num
        result += count[prefix-k]
        count[prefix]++
    }
    return result
}
