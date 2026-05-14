import java.util.*;

public class CombinationsIterative {
    /**
     * Generate all combinations of k numbers from 1 to n using iterative approach.
     * Time: O(C(n,k) * k), Space: O(C(n,k) * k) for result
     */
    public List<List<Integer>> combine(int n, int k) {
        List<List<Integer>> result = new ArrayList<>();
        int[] combo = new int[k];

        // Initialize with [1, 2, ..., k]
        for (int i = 0; i < k; i++) {
            combo[i] = i + 1;
        }

        while (true) {
            List<Integer> current = new ArrayList<>();
            for (int num : combo) {
                current.add(num);
            }
            result.add(current);

            // Find the rightmost number that can be incremented
            int i = k - 1;
            while (i >= 0 && combo[i] == n - k + i + 1) {
                i--;
            }

            if (i < 0) break;

            // Increment and reset
            combo[i]++;
            for (int j = i + 1; j < k; j++) {
                combo[j] = combo[j - 1] + 1;
            }
        }

        return result;
    }

    public static void main(String[] args) {
        CombinationsIterative sol = new CombinationsIterative();
        System.out.println(sol.combine(4, 2));
        // Output: [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
    }
}
