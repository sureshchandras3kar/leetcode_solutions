import java.util.ArrayList;
import java.util.List;

public class Main {
    public static List<Integer> spiralMatrixLayer(int[][] matrix) {
        List<Integer> result = new ArrayList<>();
        if (matrix.length == 0 || matrix[0].length == 0) {
            return result;
        }

        int m = matrix.length;
        int n = matrix[0].length;
        int layers = (Math.min(m, n) + 1) / 2;

        for (int layer = 0; layer < layers; layer++) {
            int top = layer;
            int bottom = m - 1 - layer;
            int left = layer;
            int right = n - 1 - layer;

            // Traverse right
            for (int col = left; col <= right; col++) {
                result.add(matrix[top][col]);
            }

            // Traverse down
            for (int row = top + 1; row <= bottom; row++) {
                result.add(matrix[row][right]);
            }

            // Traverse left (if there's a row remaining)
            if (top < bottom) {
                for (int col = right - 1; col >= left; col--) {
                    result.add(matrix[bottom][col]);
                }
            }

            // Traverse up (if there's a column remaining)
            if (left < right) {
                for (int row = bottom - 1; row > top; row--) {
                    result.add(matrix[row][left]);
                }
            }
        }

        return result;
    }

    public static void main(String[] args) {
        int[][] matrix = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
        List<Integer> result = spiralMatrixLayer(matrix);
        System.out.println(result);
    }
}
