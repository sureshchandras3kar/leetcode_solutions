func longestConsecutive(nums []int) int {
    numSet := map[int]bool{}
    for _, n := range nums {
        numSet[n] = true
    }
    best := 0
    for n := range numSet {
        if !numSet[n-1] {  // start of a sequence
            length := 1
            for numSet[n+length] {
                length++
            }
            if length > best {
                best = length
            }
        }
    }
    return best
}
