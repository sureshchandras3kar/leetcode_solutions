import java.util.*;

class Solution {
    public int findMinArrowShots(int[][] points) {
        if (points == null || points.length == 0) return 0;

        Arrays.sort(points, (a, b) -> {
            // Compare by end position; use long to avoid integer overflow
            if ((long)a[1] - b[1] < 0) return -1;
            if ((long)a[1] - b[1] > 0) return 1;
            return 0;
        });

        int arrows = 1;
        long lastEnd = points[0][1];

        for (int i = 1; i < points.length; i++) {
            // If current interval starts after lastEnd, no overlap
            if ((long)points[i][0] > lastEnd) {
                arrows++;
                lastEnd = points[i][1];
            }
        }

        return arrows;
    }
}
