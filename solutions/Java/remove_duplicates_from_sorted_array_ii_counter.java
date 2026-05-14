/*
Two Pointers with Counter Approach
Allow each element to appear at most twice using explicit count tracking.

Time Complexity: O(n)
Space Complexity: O(1)
*/
public class RemoveDuplicatesFromSortedArrayIICounter {
    public static int removeDuplicates(int[] nums) {
        if (nums == null || nums.length == 0) {
            return 0;
        }

        int k = 1;
        int count = 1;
        for (int i = 1; i < nums.length; i++) {
            if (nums[i] != nums[i - 1]) {
                // New element encountered, reset counter
                count = 1;
                nums[k] = nums[i];
                k++;
            } else if (count < 2) {
                // Same element but less than 2 occurrences, allow it
                count++;
                nums[k] = nums[i];
                k++;
            }
            // else: count == 2, skip this duplicate
        }

        return k;
    }

    public static void main(String[] args) {
        int[] nums1 = {1, 1, 1, 2, 2, 3};
        System.out.println(removeDuplicates(nums1));  // 5, nums = [1, 1, 2, 2, 3, _]

        int[] nums2 = {0, 0, 1, 1, 1, 1, 2, 3, 3};
        System.out.println(removeDuplicates(nums2));  // 7, nums = [0, 0, 1, 1, 2, 3, 3, _, _]

        int[] nums3 = {1};
        System.out.println(removeDuplicates(nums3));  // 1, nums = [1]

        int[] nums4 = {1, 2};
        System.out.println(removeDuplicates(nums4));  // 2, nums = [1, 2]
    }
}
