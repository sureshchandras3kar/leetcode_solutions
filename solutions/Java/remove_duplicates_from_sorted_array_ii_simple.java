/*
Simple Two Pointers Approach
Allow at most 2 occurrences by checking if current element differs from element 2 positions back.

Time Complexity: O(n)
Space Complexity: O(1)
*/
public class RemoveDuplicatesFromSortedArrayIISimple {
    public static int removeDuplicates(int[] nums) {
        if (nums == null || nums.length == 0) {
            return 0;
        }

        int k = 0;
        for (int i = 0; i < nums.length; i++) {
            // Always write first 2 elements, or if current differs from element 2 positions back
            if (k < 2 || nums[i] != nums[k - 2]) {
                nums[k] = nums[i];
                k++;
            }
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
