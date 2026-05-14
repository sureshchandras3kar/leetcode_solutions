import java.util.*;

public class Flatten {
    public boolean searchMatrix(int[][] matrix, int target) {
        if (matrix == null || matrix.length == 0) return false;

        for (int[] row : matrix) {
            if (Arrays.binarySearch(row, target) >= 0) return true;
        }
        return false;
    }
}
