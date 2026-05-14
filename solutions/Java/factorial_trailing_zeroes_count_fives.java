public class FactorialTrailingZeroesCountFives {
    public static int factorialTrailingZeroesCountFives(int n) {
        int count = 0;
        long powerOf5 = 5;
        while (powerOf5 <= n) {
            count += n / powerOf5;
            powerOf5 *= 5;
        }
        return count;
    }

    public static void main(String[] args) {
        System.out.println(factorialTrailingZeroesCountFives(5));  // 1
        System.out.println(factorialTrailingZeroesCountFives(25));  // 6
    }
}
