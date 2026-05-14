import java.util.Arrays;

public class Main {
    public static int minSubArrayLenBinarySearch(int target, int[] nums) {
        int[] prefix = new int[nums.length + 1];
        prefix[0] = 0;
        for (int i = 0; i < nums.length; i++) {
            prefix[i + 1] = prefix[i] + nums[i];
        }

        int minLength = Integer.MAX_VALUE;

        for (int right = 1; right < prefix.length; right++) {
            int needed = prefix[right] - target;
            int left = binarySearchRightmost(prefix, needed, 0, right) - 1;

            if (left >= 0 && left < right) {
                minLength = Math.min(minLength, right - left);
            }
        }

        return minLength == Integer.MAX_VALUE ? 0 : minLength;
    }

    private static int binarySearchRightmost(int[] prefix, int target, int lo, int hi) {
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (prefix[mid] <= target) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return lo;
    }

    public static void main(String[] args) {
        int[] nums = {2, 3, 1, 2, 4, 3};
        int target = 7;
        int result = minSubArrayLenBinarySearch(target, nums);
        System.out.println(result);
    }
}
