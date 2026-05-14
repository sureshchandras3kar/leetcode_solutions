public class SingleNumberIIBitCount {
    public static int singleNumberIIBitCount(int[] nums) {
        int[] bitCounts = new int[32];
        for (int num : nums) {
            for (int i = 0; i < 32; i++) {
                if ((num & (1 << i)) != 0) {
                    bitCounts[i]++;
                }
            }
        }

        int result = 0;
        for (int i = 0; i < 32; i++) {
            if (bitCounts[i] % 3 != 0) {
                result |= (1 << i);
            }
        }

        return result;
    }

    public static void main(String[] args) {
        int[] nums = {2, 2, 3, 2};
        System.out.println(singleNumberIIBitCount(nums));  // 3
    }
}
