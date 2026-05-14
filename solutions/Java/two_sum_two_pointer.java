import java.util.Arrays;

public class Main {
    public static int[] twoSumTwoPointer(int[] nums, int target) {
        int[] sorted = nums.clone();
        Arrays.sort(sorted);
        int left = 0, right = sorted.length - 1;
        while (left < right) {
            int currentSum = sorted[left] + sorted[right];
            if (currentSum == target) {
                return new int[]{sorted[left], sorted[right]};
            } else if (currentSum < target) {
                left++;
            } else {
                right--;
            }
        }
        return new int[]{};
    }

    public static void main(String[] args) {
        int[] nums = {2, 7, 11, 15};
        int target = 9;
        int[] result = twoSumTwoPointer(nums, target);
        System.out.println(Arrays.toString(result));
    }
}
