import java.util.*;

public class PermutationsSwap {
    /**
     * Generate all permutations using backtracking with swapping approach.
     * Time: O(n! * n), Space: O(n!) for result
     */
    public List<List<Integer>> permute(int[] nums) {
        List<List<Integer>> result = new ArrayList<>();
        backtrack(nums, 0, result);
        return result;
    }

    private void backtrack(int[] nums, int first, List<List<Integer>> result) {
        // Base case: all elements are placed
        if (first == nums.length) {
            List<Integer> perm = new ArrayList<>();
            for (int num : nums) {
                perm.add(num);
            }
            result.add(perm);
            return;
        }

        for (int i = first; i < nums.length; i++) {
            // Swap
            swap(nums, first, i);
            // Backtrack
            backtrack(nums, first + 1, result);
            // Swap back
            swap(nums, first, i);
        }
    }

    private void swap(int[] nums, int i, int j) {
        int temp = nums[i];
        nums[i] = nums[j];
        nums[j] = temp;
    }

    public static void main(String[] args) {
        PermutationsSwap sol = new PermutationsSwap();
        System.out.println(sol.permute(new int[]{1, 2, 3}));
        // Output: [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
    }
}
