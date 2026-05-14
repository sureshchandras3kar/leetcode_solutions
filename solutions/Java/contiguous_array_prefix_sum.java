import java.util.HashMap;
import java.util.Map;

class Solution {
    public int findMaxLength(int[] nums) {
        Map<Integer, Integer> seen = new HashMap<>();
        seen.put(0, -1);
        int prefix = 0, best = 0;
        for (int i = 0; i < nums.length; i++) {
            prefix += nums[i] == 1 ? 1 : -1;
            if (seen.containsKey(prefix)) {
                best = Math.max(best, i - seen.get(prefix));
            } else {
                seen.put(prefix, i);
            }
        }
        return best;
    }
}
