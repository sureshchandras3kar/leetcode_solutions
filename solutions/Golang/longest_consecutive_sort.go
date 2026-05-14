import "sort"

func longestConsecutive(nums []int) int {
    if len(nums) == 0 {
        return 0
    }
    sort.Ints(nums)
    best, streak := 1, 1
    for i := 1; i < len(nums); i++ {
        if nums[i] == nums[i-1] {
            continue  // skip duplicate
        }
        if nums[i] == nums[i-1]+1 {
            streak++
            if streak > best {
                best = streak
            }
        } else {
            streak = 1
        }
    }
    return best
}
