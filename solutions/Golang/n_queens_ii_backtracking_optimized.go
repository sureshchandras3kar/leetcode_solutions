package main

import "fmt"

func totalNQueens(n int) int {
    cols := make(map[int]bool)
    diag1 := make(map[int]bool)
    diag2 := make(map[int]bool)
    return backtrack(0, cols, diag1, diag2, n)
}

func backtrack(row int, cols, diag1, diag2 map[int]bool, n int) int {
    if row == n { return 1 }
    count := 0
    for col := 0; col < n; col++ {
        d1, d2 := row-col, row+col
        if !cols[col] && !diag1[d1] && !diag2[d2] {
            cols[col], diag1[d1], diag2[d2] = true, true, true
            count += backtrack(row+1, cols, diag1, diag2, n)
            delete(cols, col)
            delete(diag1, d1)
            delete(diag2, d2)
        }
    }
    return count
}
