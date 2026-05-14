impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows  = [0u16; 9];
        let mut cols  = [0u16; 9];
        let mut boxes = [0u16; 9];

        for r in 0..9 {
            for c in 0..9 {
                let d = board[r][c];
                if d == '.' { continue; }
                let bit: u16 = 1 << (d as u16 - '1' as u16);
                let b = (r / 3) * 3 + (c / 3);
                if (rows[r] & bit) != 0 || (cols[c] & bit) != 0 || (boxes[b] & bit) != 0 {
                    return false;
                }
                rows[r]  |= bit;
                cols[c]  |= bit;
                boxes[b] |= bit;
            }
        }
        true
    }
}
