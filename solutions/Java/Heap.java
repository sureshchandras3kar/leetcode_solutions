import java.util.*;

public class Heap {
    public int[][] merge(int[][] intervals) {
        PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]);
        for (int[] interval : intervals) {
            pq.offer(interval);
        }
        List<int[]> merged = new ArrayList<>();
        while (!pq.isEmpty()) {
            int[] curr = pq.poll();
            if (!merged.isEmpty() && curr[0] <= merged.get(merged.size() - 1)[1]) {
                merged.get(merged.size() - 1)[1] = Math.max(merged.get(merged.size() - 1)[1], curr[1]);
            } else {
                merged.add(curr);
            }
        }
        return merged.toArray(new int[0][0]);
    }
}
