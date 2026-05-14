import java.util.List;
import java.util.HashMap;
import java.util.Map;

public class TriangleDPTopDown {

    private Map<String, Integer> memo = new HashMap<>();
    private List<List<Integer>> triangle;

    private String encode(int row, int col) {
        return row + "," + col;
    }

    private int dfs(int row, int col) {
        if (row == triangle.size() - 1) {
            return triangle.get(row).get(col);
        }

        String key = encode(row, col);
        if (memo.containsKey(key)) {
            return memo.get(key);
        }

        int result = triangle.get(row).get(col) + Math.min(
            dfs(row + 1, col),
            dfs(row + 1, col + 1)
        );
        memo.put(key, result);
        return result;
    }

    public int minimumTotal(List<List<Integer>> triangle) {
        this.triangle = triangle;
        this.memo.clear();
        return dfs(0, 0);
    }

    public static void main(String[] args) {
        TriangleDPTopDown solution = new TriangleDPTopDown();
        List<List<Integer>> triangle = List.of(
            List.of(2),
            List.of(3, 4),
            List.of(6, 5, 7),
            List.of(4, 1, 8, 3)
        );
        System.out.println(solution.minimumTotal(triangle));  // 11
    }
}
