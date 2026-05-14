import java.util.*;

public class CombinationSumOptimized {
    /**
     * Find all unique combinations that sum to target using optimized backtracking.
     * Optimization: skip candidates that are too large.
     * Time: O(N^T), Space: O(T) for recursion depth
     */
    public List<List<Integer>> combinationSum(int[] candidates, int target) {
        Arrays.sort(candidates);  // Sort to enable pruning
        List<List<Integer>> result = new ArrayList<>();
        List<Integer> current = new ArrayList<>();
        backtrack(candidates, 0, current, target, result);
        return result;
    }

    private void backtrack(int[] candidates, int index, List<Integer> current,
                          int remaining, List<List<Integer>> result) {
        // Base case: found a valid combination
        if (remaining == 0) {
            result.add(new ArrayList<>(current));
            return;
        }

        // Explore: try each candidate from index onwards
        for (int i = index; i < candidates.length; i++) {
            int candidate = candidates[i];

            // Pruning: if candidate > remaining, all further candidates will be too large
            if (candidate > remaining) {
                break;
            }

            // Choose
            current.add(candidate);
            // Explore: can reuse the same candidate (i, not i+1)
            backtrack(candidates, i, current, remaining - candidate, result);
            // Unchoose
            current.remove(current.size() - 1);
        }
    }

    public static void main(String[] args) {
        CombinationSumOptimized sol = new CombinationSumOptimized();
        System.out.println(sol.combinationSum(new int[]{2, 3, 6, 7}, 7));
        // Output: [[2, 2, 3], [7]]
    }
}
