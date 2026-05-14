import java.util.HashMap;
import java.util.Map;

class Solution {
    public int majorityElement(int[] nums) {
        Map<Integer, Integer> counts = new HashMap<>();
        int threshold = nums.length / 2;
        for (int num : nums) {
            int next = counts.getOrDefault(num, 0) + 1;
            counts.put(num, next);
            if (next > threshold) return num;
        }
        return nums[0];
    }
}
