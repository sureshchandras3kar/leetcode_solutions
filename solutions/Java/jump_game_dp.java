public class JumpGameDP {

    /**
     * Dynamic programming approach: forward pass tracking the furthest
     * reachable index. If we can ever reach the end index, return true.
     *
     * Time: O(n), Space: O(1)
     */
    public static boolean canJumpDP(int[] nums) {
        int maxReach = 0;
        for (int i = 0; i < nums.length; i++) {
            if (i > maxReach) {
                return false;
            }
            maxReach = Math.max(maxReach, i + nums[i]);
            if (maxReach >= nums.length - 1) {
                return true;
            }
        }
        return false;
    }

    public static void main(String[] args) {
        int[] nums1 = {2, 3, 1, 1, 4};
        int[] nums2 = {3, 2, 1, 0, 4};
        int[] nums3 = {0};
        int[] nums4 = {2, 0, 0};
        int[] nums5 = {0, 2, 3};

        System.out.println(canJumpDP(nums1));  // true
        System.out.println(canJumpDP(nums2));  // false
        System.out.println(canJumpDP(nums3));  // true
        System.out.println(canJumpDP(nums4));  // true
        System.out.println(canJumpDP(nums5));  // false
    }
}
