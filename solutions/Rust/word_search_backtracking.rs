pub fn exist(board: &mut Vec<Vec<char>>, word: String) -> bool {
    if board.is_empty() { return false; }

    fn backtrack(board: &mut Vec<Vec<char>>, word: &str, row: usize, col: usize, index: usize) -> bool {
        if index == word.len() { return true; }
        if row >= board.len() || col >= board[0].len() { return false; }
        if board[row][col] != word.chars().nth(index).unwrap() { return false; }

        board[row][col] = '#';
        let found = (row > 0 && backtrack(board, word, row - 1, col, index + 1)) ||
                    (row + 1 < board.len() && backtrack(board, word, row + 1, col, index + 1)) ||
                    (col > 0 && backtrack(board, word, row, col - 1, index + 1)) ||
                    (col + 1 < board[0].len() && backtrack(board, word, row, col + 1, index + 1));
        board[row][col] = word.chars().nth(index).unwrap();
        found
    }

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if backtrack(board, &word, i, j, 0) { return true; }
        }
    }
    false
}
