function isValidSudoku(board) {
    const rows  = new Array(9).fill(0);
    const cols  = new Array(9).fill(0);
    const boxes = new Array(9).fill(0);

    for (let r = 0; r < 9; r++) {
        for (let c = 0; c < 9; c++) {
            const d = board[r][c];
            if (d === '.') continue;
            const bit = 1 << (parseInt(d) - 1);
            const box = Math.floor(r / 3) * 3 + Math.floor(c / 3);
            if ((rows[r] & bit) || (cols[c] & bit) || (boxes[box] & bit)) return false;
            rows[r]  |= bit;
            cols[c]  |= bit;
            boxes[box] |= bit;
        }
    }
    return true;
}
