import java.util.*;

class Solution {
    public int[][] insert(int[][] intervals, int[] newInterval) {
        List<int[]> result = new ArrayList<>();
        int i = 0;
        int newStart = newInterval[0];
        int newEnd = newInterval[1];

        // Phase 1: Add all intervals that end before newInterval starts
        while (i < intervals.length && intervals[i][1] < newStart) {
            result.add(intervals[i]);
            i++;
        }

        // Phase 2: Merge all overlapping intervals
        while (i < intervals.length && intervals[i][0] <= newEnd) {
            newStart = Math.min(newStart, intervals[i][0]);
            newEnd = Math.max(newEnd, intervals[i][1]);
            i++;
        }
        result.add(new int[]{newStart, newEnd});

        // Phase 3: Add remaining intervals
        while (i < intervals.length) {
            result.add(intervals[i]);
            i++;
        }

        return result.toArray(new int[result.size()][]);
    }
}
