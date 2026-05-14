package main

import "sort"

func searchMatrix(matrix [][]int, target int) bool {
    for _, row := range matrix {
        if sort.SearchInts(row, target) < len(row) {
            return true
        }
    }
    return false
}
