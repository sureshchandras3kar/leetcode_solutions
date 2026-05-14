impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }
        
        let rows = board.len();
        let cols = board[0].len();
        
        // Mark border-connected O's
        for i in 0..rows {
            if board[i][0] == 'O' {
                Self::dfs(board, i, 0, rows, cols);
            }
            if board[i][cols - 1] == 'O' {
                Self::dfs(board, i, cols - 1, rows, cols);
            }
        }
        
        for j in 0..cols {
            if board[0][j] == 'O' {
                Self::dfs(board, 0, j, rows, cols);
            }
            if board[rows - 1][j] == 'O' {
                Self::dfs(board, rows - 1, j, rows, cols);
            }
        }
        
        // Flip and restore
        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                } else if board[i][j] == '#' {
                    board[i][j] = 'O';
                }
            }
        }
    }
    
    fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
        if r >= rows || c >= cols || board[r][c] != 'O' {
            return;
        }
        board[r][c] = '#';
        if r > 0 {
            Self::dfs(board, r - 1, c, rows, cols);
        }
        Self::dfs(board, r + 1, c, rows, cols);
        if c > 0 {
            Self::dfs(board, r, c - 1, rows, cols);
        }
        Self::dfs(board, r, c + 1, rows, cols);
    }
}

pub struct Solution;
