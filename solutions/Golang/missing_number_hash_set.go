func missingNumber(nums []int) int {
    seen := map[int]bool{}
    for _, num := range nums {
        seen[num] = true
    }
    for value := 0; value <= len(nums); value++ {
        if !seen[value] {
            return value
        }
    }
    return -1
}
