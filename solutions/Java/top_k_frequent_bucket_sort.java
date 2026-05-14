import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

class Solution {
    public int[] topKFrequent(int[] nums, int k) {
        Map<Integer, Integer> freq = new HashMap<>();
        for (int num : nums) freq.merge(num, 1, Integer::sum);

        int n = nums.length;
        @SuppressWarnings("unchecked")
        List<Integer>[] buckets = new List[n + 1];
        for (int i = 0; i <= n; i++) buckets[i] = new ArrayList<>();

        for (Map.Entry<Integer, Integer> e : freq.entrySet()) {
            buckets[e.getValue()].add(e.getKey());
        }

        int[] result = new int[k];
        int idx = 0;
        for (int i = n; i >= 1 && idx < k; i--) {
            for (int num : buckets[i]) {
                result[idx++] = num;
                if (idx == k) break;
            }
        }
        return result;
    }
}
