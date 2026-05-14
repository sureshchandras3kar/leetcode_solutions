package main

func exist(board [][]byte, word string) bool {
    if len(board) == 0 { return false }

    var backtrack func(int, int, int) bool
    backtrack = func(row, col, index int) bool {
        if index == len(word) { return true }
        if row < 0 || row >= len(board) || col < 0 || col >= len(board[0]) { return false }
        if board[row][col] != word[index] { return false }

        board[row][col] = '#'
        found := backtrack(row+1, col, index+1) ||
                 backtrack(row-1, col, index+1) ||
                 backtrack(row, col+1, index+1) ||
                 backtrack(row, col-1, index+1)
        board[row][col] = word[index]
        return found
    }

    for i := 0; i < len(board); i++ {
        for j := 0; j < len(board[0]); j++ {
            if backtrack(i, j, 0) { return true }
        }
    }
    return false
}
