public class RemoveDuplicatesFromSortedArrayTwoPointer {
    /*
    Two-Pointer In-Place Approach
    Remove duplicates from sorted array in-place and return the length of the new array.

    Time Complexity: O(n)
    Space Complexity: O(1)
    */
    public static int removeDuplicates(int[] nums) {
        if (nums == null || nums.length == 0) {
            return 0;
        }

        int k = 1;  // First element is always unique
        for (int i = 1; i < nums.length; i++) {
            if (nums[i] != nums[i - 1]) {
                nums[k] = nums[i];
                k++;
            }
        }

        return k;
    }

    public static void main(String[] args) {
        int[] nums1 = {1, 1, 2};
        System.out.println(removeDuplicates(nums1));  // 2, nums = [1, 2, _]

        int[] nums2 = {0, 0, 1, 1, 1, 2, 2, 3, 3, 4};
        System.out.println(removeDuplicates(nums2));  // 5, nums = [0, 1, 2, 3, 4, _, _, _, _, _]
    }
}
