func isValidSudoku(board [][]byte) bool {
    var rows, cols, boxes [9]int

    for r := 0; r < 9; r++ {
        for c := 0; c < 9; c++ {
            d := board[r][c]
            if d == '.' {
                continue
            }
            bit := 1 << (d - '1')
            b := (r/3)*3 + (c / 3)
            if rows[r]&bit != 0 || cols[c]&bit != 0 || boxes[b]&bit != 0 {
                return false
            }
            rows[r] |= bit
            cols[c] |= bit
            boxes[b] |= bit
        }
    }
    return true
}
