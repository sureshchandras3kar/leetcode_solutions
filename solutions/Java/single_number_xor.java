public class SingleNumberXor {
    public static int singleNumberXor(int[] nums) {
        int result = 0;
        for (int num : nums) {
            result ^= num;
        }
        return result;
    }

    public static void main(String[] args) {
        int[] nums = {2, 2, 1};
        System.out.println(singleNumberXor(nums));  // 1
    }
}
