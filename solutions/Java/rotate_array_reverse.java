public class Main {
    public static void rotateArrayReverse(int[] nums, int k) {
        /**
         * Rotate array in-place using reverse trick.
         *
         * Time: O(n) | Space: O(1)
         *
         * Approach: Reverse the entire array, then reverse first k elements,
         * then reverse remaining n-k elements. This achieves rotation without
         * extra space.
         */
        if (nums == null || nums.length == 0 || k == 0) {
            return;
        }

        int n = nums.length;
        k = k % n;  // Handle k > n
        if (k == 0) {
            return;
        }

        reverse(nums, 0, n - 1);
        reverse(nums, 0, k - 1);
        reverse(nums, k, n - 1);
    }

    private static void reverse(int[] nums, int start, int end) {
        while (start < end) {
            int temp = nums[start];
            nums[start] = nums[end];
            nums[end] = temp;
            start++;
            end--;
        }
    }

    public static void main(String[] args) {
        int[] nums = {1, 2, 3, 4, 5};
        rotateArrayReverse(nums, 2);
        for (int num : nums) {
            System.out.print(num + " ");
        }
        System.out.println();  // [4, 5, 1, 2, 3]
    }
}
