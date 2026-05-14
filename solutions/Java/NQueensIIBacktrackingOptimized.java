import java.util.*;

class Solution {
    public int totalNQueens(int n) {
        Set<Integer> cols = new HashSet<>();
        Set<Integer> diag1 = new HashSet<>();
        Set<Integer> diag2 = new HashSet<>();
        return backtrack(0, n, cols, diag1, diag2);
    }

    private int backtrack(int row, int n, Set<Integer> cols, Set<Integer> diag1, Set<Integer> diag2) {
        if (row == n) {
            return 1;
        }

        int count = 0;
        for (int col = 0; col < n; col++) {
            int d1 = row - col;
            int d2 = row + col;

            if (cols.contains(col) || diag1.contains(d1) || diag2.contains(d2)) {
                continue;
            }

            cols.add(col);
            diag1.add(d1);
            diag2.add(d2);

            count += backtrack(row + 1, n, cols, diag1, diag2);

            cols.remove(col);
            diag1.remove(d1);
            diag2.remove(d2);
        }

        return count;
    }
}
