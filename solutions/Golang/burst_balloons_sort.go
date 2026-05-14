package main

import "sort"

func findMinArrowShots(points [][]int) int {
    if len(points) == 0 { return 0 }
    sort.Slice(points, func(i, j int) bool {
        if points[i][0] != points[j][0] { return points[i][0] < points[j][0] }
        return points[i][1] < points[j][1]
    })
    arrows := 1
    lastEnd := int64(points[0][1])
    for i := 1; i < len(points); i++ {
        if int64(points[i][0]) > lastEnd {
            arrows++
            lastEnd = int64(points[i][1])
        }
    }
    return arrows
}
