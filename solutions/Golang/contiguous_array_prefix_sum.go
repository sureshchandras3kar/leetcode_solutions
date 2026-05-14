func findMaxLength(nums []int) int {
    seen := map[int]int{0: -1}
    prefix, best := 0, 0
    for i, num := range nums {
        if num == 1 {
            prefix++
        } else {
            prefix--
        }
        if first, ok := seen[prefix]; ok {
            if i-first > best {
                best = i - first
            }
        } else {
            seen[prefix] = i
        }
    }
    return best
}
