public class PowxnFastExponentiation {
    public static double powxnFastExponentiation(double x, int n) {
        if (n == 0) return 1.0;

        long N = n;
        if (N < 0) {
            x = 1 / x;
            N = -N;
        }

        double result = 1.0;
        while (N > 0) {
            if (N % 2 == 1) {
                result *= x;
            }
            x *= x;
            N /= 2;
        }

        return result;
    }

    public static void main(String[] args) {
        System.out.println(powxnFastExponentiation(2.0, 10));  // 1024.0
    }
}
