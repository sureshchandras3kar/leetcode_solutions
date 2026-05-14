import java.util.HashSet;
import java.util.Set;

class Solution {
    public int missingNumber(int[] nums) {
        Set<Integer> seen = new HashSet<>();
        for (int num : nums) seen.add(num);
        for (int value = 0; value <= nums.length; value++) {
            if (!seen.contains(value)) return value;
        }
        return -1;
    }
}
