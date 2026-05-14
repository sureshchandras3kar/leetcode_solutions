pub fn total_n_queens(n: i32) -> i32 {
    let n = n as usize;
    let mut board = vec![-1; n];

    fn is_safe(board: &[i32], row: usize, col: i32) -> bool {
        for i in 0..row {
            if board[i] == col || (board[i] - col).abs() == (i as i32 - row as i32).abs() {
                return false;
            }
        }
        true
    }

    fn backtrack(board: &mut Vec<i32>, row: usize, n: usize) -> i32 {
        if row == n { return 1; }
        let mut count = 0;
        for col in 0..n {
            if is_safe(board, row, col as i32) {
                board[row] = col as i32;
                count += backtrack(board, row + 1, n);
            }
        }
        count
    }

    backtrack(&mut board, 0, n) as i32
}
