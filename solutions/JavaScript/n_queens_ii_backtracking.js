function totalNQueens(n) {
    const board = new Array(n).fill(-1);

    function isSafe(row, col) {
        for (let i = 0; i < row; i++) {
            if (board[i] === col || Math.abs(board[i] - col) === Math.abs(i - row))
                return false;
        }
        return true;
    }

    function backtrack(row) {
        if (row === n) return 1;
        let count = 0;
        for (let col = 0; col < n; col++) {
            if (isSafe(row, col)) {
                board[row] = col;
                count += backtrack(row + 1);
            }
        }
        return count;
    }

    return backtrack(0);
}
