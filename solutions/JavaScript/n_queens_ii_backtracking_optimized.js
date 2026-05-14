function totalNQueens(n) {
    function backtrack(row, cols, diag1, diag2) {
        if (row === n) return 1;
        let count = 0;
        for (let col = 0; col < n; col++) {
            const d1 = row - col, d2 = row + col;
            if (!cols.has(col) && !diag1.has(d1) && !diag2.has(d2)) {
                cols.add(col);
                diag1.add(d1);
                diag2.add(d2);
                count += backtrack(row + 1, cols, diag1, diag2);
                cols.delete(col);
                diag1.delete(d1);
                diag2.delete(d2);
            }
        }
        return count;
    }
    return backtrack(0, new Set(), new Set(), new Set());
}
