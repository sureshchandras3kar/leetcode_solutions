/**
 * Rob houses with maximum money using space-optimized approach.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 */
public class HouseRobberSpaceOptimized {
    public static int houseRobberSpaceOptimized(int[] nums) {
        if (nums.length == 0) return 0;
        if (nums.length == 1) return nums[0];

        int prev2 = nums[0];
        int prev1 = Math.max(nums[0], nums[1]);

        for (int i = 2; i < nums.length; i++) {
            int current = Math.max(nums[i] + prev2, prev1);
            prev2 = prev1;
            prev1 = current;
        }

        return prev1;
    }

    public static void main(String[] args) {
        System.out.println(houseRobberSpaceOptimized(new int[]{1, 2, 3, 1}));      // 4
        System.out.println(houseRobberSpaceOptimized(new int[]{2, 7, 9, 3, 1}));   // 12
    }
}
