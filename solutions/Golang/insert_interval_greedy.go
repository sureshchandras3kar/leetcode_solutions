package main

func insert(intervals [][]int, newInterval []int) [][]int {
    result := [][]int{}
    i, new_start, new_end := 0, newInterval[0], newInterval[1]
    for i < len(intervals) && intervals[i][1] < new_start {
        result = append(result, intervals[i])
        i++
    }
    for i < len(intervals) && intervals[i][0] <= new_end {
        if intervals[i][0] < new_start { new_start = intervals[i][0] }
        if intervals[i][1] > new_end { new_end = intervals[i][1] }
        i++
    }
    result = append(result, []int{new_start, new_end})
    for i < len(intervals) {
        result = append(result, intervals[i])
        i++
    }
    return result
}
