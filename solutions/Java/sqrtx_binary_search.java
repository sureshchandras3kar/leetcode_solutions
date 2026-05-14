public class SqrtxBinarySearch {
    public static int sqrtxBinarySearch(int x) {
        if (x < 2) return x;

        long left = 2, right = x / 2;
        while (left <= right) {
            long mid = (left + right) / 2;
            if (mid * mid == x) {
                return (int) mid;
            } else if (mid * mid < x) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        return (int) right;
    }

    public static void main(String[] args) {
        System.out.println(sqrtxBinarySearch(4));  // 2
        System.out.println(sqrtxBinarySearch(8));  // 2
    }
}
