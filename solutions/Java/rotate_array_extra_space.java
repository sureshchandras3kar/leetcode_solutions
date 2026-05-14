public class Main {
    public static void rotateArrayExtraSpace(int[] nums, int k) {
        /**
         * Rotate array using extra space.
         *
         * Time: O(n) | Space: O(n)
         *
         * Approach: Create a new array where element at index i goes to index
         * (i + k) % n. Copy back to original array.
         */
        if (nums == null || nums.length == 0 || k == 0) {
            return;
        }

        int n = nums.length;
        k = k % n;  // Handle k > n
        if (k == 0) {
            return;
        }

        // Create rotated result
        int[] rotated = new int[n];
        for (int i = 0; i < n; i++) {
            rotated[(i + k) % n] = nums[i];
        }

        // Copy back to original array
        for (int i = 0; i < n; i++) {
            nums[i] = rotated[i];
        }
    }

    public static void main(String[] args) {
        int[] nums = {1, 2, 3, 4, 5};
        rotateArrayExtraSpace(nums, 2);
        for (int num : nums) {
            System.out.print(num + " ");
        }
        System.out.println();  // [4, 5, 1, 2, 3]
    }
}
