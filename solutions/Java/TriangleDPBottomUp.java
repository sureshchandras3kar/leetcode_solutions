import java.util.List;

public class TriangleDPBottomUp {

    public static int minimumTotal(List<List<Integer>> triangle) {
        /*
        DP Bottom-Up approach for Triangle problem.
        Builds the solution from the bottom row upward.
        Time: O(n²), Space: O(n)
        */
        int n = triangle.size();
        int[] dp = new int[n];

        // Copy last row
        List<Integer> lastRow = triangle.get(n - 1);
        for (int i = 0; i < lastRow.size(); i++) {
            dp[i] = lastRow.get(i);
        }

        // Work backwards from the second-to-last row to the top
        for (int i = n - 2; i >= 0; --i) {
            List<Integer> currentRow = triangle.get(i);
            for (int j = 0; j < currentRow.size(); ++j) {
                dp[j] = currentRow.get(j) + Math.min(dp[j], dp[j + 1]);
            }
        }

        return dp[0];
    }

    public static void main(String[] args) {
        List<List<Integer>> triangle = List.of(
            List.of(2),
            List.of(3, 4),
            List.of(6, 5, 7),
            List.of(4, 1, 8, 3)
        );
        System.out.println(minimumTotal(triangle));  // 11
    }
}
