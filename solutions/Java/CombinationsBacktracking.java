import java.util.*;

public class CombinationsBacktracking {
    /**
     * Generate all combinations of k numbers from 1 to n using backtracking.
     * Time: O(C(n,k) * k), Space: O(C(n,k) * k) for result
     */
    public List<List<Integer>> combine(int n, int k) {
        List<List<Integer>> result = new ArrayList<>();
        List<Integer> current = new ArrayList<>();
        backtrack(n, k, 1, current, result);
        return result;
    }

    private void backtrack(int n, int k, int start, List<Integer> current,
                          List<List<Integer>> result) {
        // Base case: we've selected k numbers
        if (current.size() == k) {
            result.add(new ArrayList<>(current));
            return;
        }

        // Explore: try each remaining number
        for (int i = start; i <= n; i++) {
            // Choose
            current.add(i);
            // Explore
            backtrack(n, k, i + 1, current, result);
            // Unchoose
            current.remove(current.size() - 1);
        }
    }

    public static void main(String[] args) {
        CombinationsBacktracking sol = new CombinationsBacktracking();
        System.out.println(sol.combine(4, 2));
        // Output: [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
    }
}
