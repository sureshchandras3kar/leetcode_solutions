class Solution {
    public boolean isValidSudoku(char[][] board) {
        int[] rows  = new int[9];
        int[] cols  = new int[9];
        int[] boxes = new int[9];

        for (int r = 0; r < 9; r++) {
            for (int c = 0; c < 9; c++) {
                char d = board[r][c];
                if (d == '.') continue;
                int bit = 1 << (d - '1');
                int box = (r / 3) * 3 + (c / 3);
                if ((rows[r] & bit) != 0 || (cols[c] & bit) != 0 || (boxes[box] & bit) != 0)
                    return false;
                rows[r]  |= bit;
                cols[c]  |= bit;
                boxes[box] |= bit;
            }
        }
        return true;
    }
}
