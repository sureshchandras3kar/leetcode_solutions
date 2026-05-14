import java.util.*;

public class Sort {
    public int findMinArrowShots(int[][] points) {
        Arrays.sort(points);
        int arrows = 1;
        long last_end = points[0][1];
        for (int i = 1; i < points.length; i++) {
            if ((long)points[i][0] > last_end) {
                arrows++;
                last_end = points[i][1];
            }
        }
        return arrows;
    }
}
