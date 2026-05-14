public class ReverseBitsIterate {
    public static int reverseBitsIterate(int n) {
        int result = 0;
        for (int i = 0; i < 32; i++) {
            result = (result << 1) | (n & 1);
            n >>>= 1;
        }
        return result;
    }

    public static void main(String[] args) {
        System.out.println(reverseBitsIterate(43261596));
    }
}
