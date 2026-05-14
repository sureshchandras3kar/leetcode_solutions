func isValidSudoku(board [][]byte) bool {
    var rows, cols, boxes [9]map[byte]bool
    for i := 0; i < 9; i++ {
        rows[i]  = map[byte]bool{}
        cols[i]  = map[byte]bool{}
        boxes[i] = map[byte]bool{}
    }

    for r := 0; r < 9; r++ {
        for c := 0; c < 9; c++ {
            d := board[r][c]
            if d == '.' {
                continue
            }
            b := (r/3)*3 + (c / 3)
            if rows[r][d] || cols[c][d] || boxes[b][d] {
                return false
            }
            rows[r][d] = true
            cols[c][d] = true
            boxes[b][d] = true
        }
    }
    return true
}
