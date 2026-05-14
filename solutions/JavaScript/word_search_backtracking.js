function exist(board, word) {
    if (!board || !board.length) return false;

    function backtrack(row, col, index) {
        if (index === word.length) return true;
        if (row < 0 || row >= board.length || col < 0 || col >= board[0].length) return false;
        if (board[row][col] !== word[index]) return false;

        board[row][col] = '#';
        const found = backtrack(row + 1, col, index + 1) ||
                      backtrack(row - 1, col, index + 1) ||
                      backtrack(row, col + 1, index + 1) ||
                      backtrack(row, col - 1, index + 1);
        board[row][col] = word[index];
        return found;
    }

    for (let i = 0; i < board.length; i++) {
        for (let j = 0; j < board[0].length; j++) {
            if (backtrack(i, j, 0)) return true;
        }
    }
    return false;
}
