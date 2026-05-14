use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows:  [HashSet<char>; 9] = Default::default();
        let mut cols:  [HashSet<char>; 9] = Default::default();
        let mut boxes: [HashSet<char>; 9] = Default::default();

        for r in 0..9 {
            for c in 0..9 {
                let d = board[r][c];
                if d == '.' { continue; }
                let b = (r / 3) * 3 + (c / 3);
                if !rows[r].insert(d) || !cols[c].insert(d) || !boxes[b].insert(d) {
                    return false;
                }
            }
        }
        true
    }
}
