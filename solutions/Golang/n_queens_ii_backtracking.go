package main

func totalNQueens(n int) int {
    board := make([]int, n)
    for i := range board { board[i] = -1 }
    return backtrack(board, 0, n)
}

func isSafe(board []int, row, col int) bool {
    for i := 0; i < row; i++ {
        if board[i] == col {
            return false
        }
        if abs(board[i]-col) == abs(i-row) {
            return false
        }
    }
    return true
}

func backtrack(board []int, row, n int) int {
    if row == n { return 1 }
    count := 0
    for col := 0; col < n; col++ {
        if isSafe(board, row, col) {
            board[row] = col
            count += backtrack(board, row+1, n)
        }
    }
    return count
}

func abs(x int) int { if x < 0 { return -x }; return x }
