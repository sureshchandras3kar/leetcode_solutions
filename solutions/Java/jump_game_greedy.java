public class JumpGameGreedy {

    /**
     * Greedy approach: work backwards from the end to find the furthest index
     * we can reach. If we can reach index 0, we can reach the end.
     *
     * Time: O(n), Space: O(1)
     */
    public static boolean canJumpGreedy(int[] nums) {
        int lastGoodIndex = nums.length - 1;
        for (int i = nums.length - 2; i >= 0; i--) {
            if (i + nums[i] >= lastGoodIndex) {
                lastGoodIndex = i;
            }
        }
        return lastGoodIndex == 0;
    }

    public static void main(String[] args) {
        int[] nums1 = {2, 3, 1, 1, 4};
        int[] nums2 = {3, 2, 1, 0, 4};
        int[] nums3 = {0};
        int[] nums4 = {2, 0, 0};
        int[] nums5 = {0, 2, 3};

        System.out.println(canJumpGreedy(nums1));  // true
        System.out.println(canJumpGreedy(nums2));  // false
        System.out.println(canJumpGreedy(nums3));  // true
        System.out.println(canJumpGreedy(nums4));  // true
        System.out.println(canJumpGreedy(nums5));  // false
    }
}
