package main

func solveDFS(board [][]byte) {
    if len(board) == 0 || len(board[0]) == 0 {
        return
    }
    
    rows, cols := len(board), len(board[0])
    
    var dfs func(int, int)
    dfs = func(r, c int) {
        if r < 0 || r >= rows || c < 0 || c >= cols || board[r][c] != 'O' {
            return
        }
        board[r][c] = '#'
        dfs(r+1, c)
        dfs(r-1, c)
        dfs(r, c+1)
        dfs(r, c-1)
    }
    
    for i := 0; i < rows; i++ {
        if board[i][0] == 'O' {
            dfs(i, 0)
        }
        if board[i][cols-1] == 'O' {
            dfs(i, cols-1)
        }
    }
    
    for j := 0; j < cols; j++ {
        if board[0][j] == 'O' {
            dfs(0, j)
        }
        if board[rows-1][j] == 'O' {
            dfs(rows-1, j)
        }
    }
    
    for i := 0; i < rows; i++ {
        for j := 0; j < cols; j++ {
            if board[i][j] == 'O' {
                board[i][j] = 'X'
            } else if board[i][j] == '#' {
                board[i][j] = 'O'
            }
        }
    }
}
