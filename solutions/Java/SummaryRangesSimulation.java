import java.util.ArrayList;
import java.util.List;

public class SummaryRangesSimulation {
    public List<String> summaryRanges(int[] nums) {
        /*
        Simulation approach: iterate through and build ranges as we go.
        Time: O(n), Space: O(1) excluding output
        */
        List<String> result = new ArrayList<>();
        if (nums.length == 0) return result;

        long start = nums[0];

        for (int i = 1; i < nums.length; i++) {
            if (nums[i] != nums[i - 1] + 1) {
                if (start == nums[i - 1]) {
                    result.add(String.valueOf(start));
                } else {
                    result.add(start + "->" + nums[i - 1]);
                }
                start = nums[i];
            }
        }

        if (start == nums[nums.length - 1]) {
            result.add(String.valueOf(start));
        } else {
            result.add(start + "->" + nums[nums.length - 1]);
        }

        return result;
    }
}
