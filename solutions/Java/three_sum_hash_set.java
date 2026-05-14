import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Main {
    public static List<List<Integer>> threeSumHashSet(int[] nums) {
        Arrays.sort(nums);
        List<List<Integer>> result = new ArrayList<>();
        int n = nums.length;

        for (int i = 0; i < n - 2; i++) {
            if (i > 0 && nums[i] == nums[i - 1]) continue;
            Set<Integer> seen = new HashSet<>();
            for (int j = i + 1; j < n; j++) {
                int need = -(nums[i] + nums[j]);
                if (seen.contains(need)) {
                    List<Integer> triplet = Arrays.asList(nums[i], need, nums[j]);
                    if (!result.contains(triplet)) {
                        result.add(triplet);
                    }
                }
                seen.add(nums[j]);
            }
        }

        return result;
    }

    public static void main(String[] args) {
        int[] nums = {-1, 0, 1, 2, -1, -4};
        System.out.println(threeSumHashSet(nums)); // [[-1, -1, 2], [-1, 0, 1]]
    }
}
