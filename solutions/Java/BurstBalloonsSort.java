import java.util.*;

class Solution {
    public int findMinArrowShots(int[][] points) {
        if (points == null || points.length == 0) return 0;

        // Sort by end position (ascending)
        Arrays.sort(points, (a, b) -> {
            // Use long to avoid integer overflow with edge values
            if ((long)a[1] - b[1] < 0) return -1;
            if ((long)a[1] - b[1] > 0) return 1;
            return 0;
        });

        int count = 1;
        long lastArrowPos = points[0][1];

        for (int i = 1; i < points.length; i++) {
            // If current interval doesn't overlap with last arrow position
            if ((long)points[i][0] > lastArrowPos) {
                count++;
                lastArrowPos = points[i][1];
            }
        }

        return count;
    }
}
