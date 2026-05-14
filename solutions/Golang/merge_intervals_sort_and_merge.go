package main

import "sort"

func merge(intervals [][]int) [][]int {
    if len(intervals) == 0 { return [][]int{} }
    sort.Slice(intervals, func(i, j int) bool { return intervals[i][0] < intervals[j][0] })
    merged := [][]int{intervals[0]}
    for i := 1; i < len(intervals); i++ {
        last := &merged[len(merged)-1]
        if intervals[i][0] <= (*last)[1] {
            (*last)[1] = max((*last)[1], intervals[i][1])
        } else {
            merged = append(merged, intervals[i])
        }
    }
    return merged
}

func max(a, b int) int { if a > b { return a }; return b }
