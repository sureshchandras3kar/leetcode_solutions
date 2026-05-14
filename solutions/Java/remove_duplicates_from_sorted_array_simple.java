public class RemoveDuplicatesFromSortedArraySimple {
    /*
    Simple Pass Approach
    Remove duplicates by iterating and comparing consecutive elements.

    Time Complexity: O(n)
    Space Complexity: O(1)
    */
    public static int removeDuplicates(int[] nums) {
        if (nums == null || nums.length == 0) {
            return 0;
        }

        int writeIdx = 0;
        for (int readIdx = 1; readIdx < nums.length; readIdx++) {
            if (nums[readIdx] != nums[writeIdx]) {
                writeIdx++;
                nums[writeIdx] = nums[readIdx];
            }
        }

        return writeIdx + 1;
    }

    public static void main(String[] args) {
        int[] nums1 = {1, 1, 2};
        System.out.println(removeDuplicates(nums1));  // 2, nums = [1, 2, _]

        int[] nums2 = {0, 0, 1, 1, 1, 2, 2, 3, 3, 4};
        System.out.println(removeDuplicates(nums2));  // 5, nums = [0, 1, 2, 3, 4, _, _, _, _, _]
    }
}
