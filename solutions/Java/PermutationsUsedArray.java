import java.util.*;

public class PermutationsUsedArray {
    /**
     * Generate all permutations using backtracking with used array.
     * Time: O(n! * n), Space: O(n!) for result
     */
    public List<List<Integer>> permute(int[] nums) {
        List<List<Integer>> result = new ArrayList<>();
        boolean[] used = new boolean[nums.length];
        List<Integer> current = new ArrayList<>();
        backtrack(nums, used, current, result);
        return result;
    }

    private void backtrack(int[] nums, boolean[] used, List<Integer> current,
                          List<List<Integer>> result) {
        // Base case: we've used all numbers
        if (current.size() == nums.length) {
            result.add(new ArrayList<>(current));
            return;
        }

        for (int i = 0; i < nums.length; i++) {
            if (used[i]) continue;

            // Choose
            current.add(nums[i]);
            used[i] = true;
            // Explore
            backtrack(nums, used, current, result);
            // Unchoose
            current.remove(current.size() - 1);
            used[i] = false;
        }
    }

    public static void main(String[] args) {
        PermutationsUsedArray sol = new PermutationsUsedArray();
        System.out.println(sol.permute(new int[]{1, 2, 3}));
        // Output: [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
    }
}
