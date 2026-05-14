import java.util.*;

public class BacktrackingOptimized {
    public int totalNQueens(int n) {
        return backtrack(0, new HashSet<>(), new HashSet<>(), new HashSet<>(), n);
    }

    private int backtrack(int row, Set<Integer> cols, Set<Integer> diag1, Set<Integer> diag2, int n) {
        if (row == n) return 1;
        int count = 0;
        for (int col = 0; col < n; col++) {
            if (!cols.contains(col) && !diag1.contains(row - col) && !diag2.contains(row + col)) {
                cols.add(col);
                diag1.add(row - col);
                diag2.add(row + col);
                count += backtrack(row + 1, cols, diag1, diag2, n);
                cols.remove(col);
                diag1.remove(row - col);
                diag2.remove(row + col);
            }
        }
        return count;
    }
}
