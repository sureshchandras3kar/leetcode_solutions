import java.util.HashSet;
import java.util.Set;

class Solution {
    public int longestConsecutive(int[] nums) {
        Set<Integer> numSet = new HashSet<>();
        for (int n : nums) numSet.add(n);
        int best = 0;
        for (int n : numSet) {
            if (!numSet.contains(n - 1)) {  // start of a sequence
                int length = 1;
                while (numSet.contains(n + length)) {
                    length++;
                }
                best = Math.max(best, length);
            }
        }
        return best;
    }
}
