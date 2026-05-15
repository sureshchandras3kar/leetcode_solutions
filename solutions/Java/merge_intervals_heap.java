import java.util.ArrayList;
import java.util.List;
import java.util.PriorityQueue;

class Solution {
    public int[][] merge(int[][] intervals) {
        if (intervals.length == 0) return new int[0][0];

        PriorityQueue<int[]> heap = new PriorityQueue<>((a, b) -> Integer.compare(a[0], b[0]));
        for (int[] interval : intervals) {
            heap.offer(interval);
        }

        List<int[]> merged = new ArrayList<>();
        while (!heap.isEmpty()) {
            int[] current = heap.poll();
            if (!merged.isEmpty() && current[0] <= merged.get(merged.size() - 1)[1]) {
                int[] last = merged.get(merged.size() - 1);
                last[1] = Math.max(last[1], current[1]);
            } else {
                merged.add(current);
            }
        }

        return merged.toArray(new int[0][]);
    }
}

int[][] result = new Solution().merge(new int[][]{{1, 3}, {2, 6}, {8, 10}});
for (int[] interval : result) {
    System.out.println(java.util.Arrays.toString(interval));
}
