import java.util.*;

/**
 * Find length of longest increasing subsequence using binary search O(n log n).
 *
 * Time Complexity: O(n log n)
 * Space Complexity: O(n)
 */
public class LIsBinarySearch {
    public static int lisBinarySearch(int[] nums) {
        if (nums.length == 0) return 0;

        List<Integer> tails = new ArrayList<>();

        for (int num : nums) {
            int pos = Collections.binarySearch(tails, num);
            if (pos < 0) {
                pos = -(pos + 1);
            }

            if (pos == tails.size()) {
                tails.add(num);
            } else {
                tails.set(pos, num);
            }
        }

        return tails.size();
    }

    public static void main(String[] args) {
        System.out.println(lisBinarySearch(new int[]{10, 9, 2, 5, 3, 7, 101, 18}));  // 4
    }
}
