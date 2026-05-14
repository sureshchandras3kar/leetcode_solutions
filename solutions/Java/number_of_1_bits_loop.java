public class NumberOf1BitsLoop {
    public static int numberOf1BitsLoop(int n) {
        int count = 0;
        while (n != 0) {
            count += n & 1;
            n >>>= 1;
        }
        return count;
    }

    public static void main(String[] args) {
        System.out.println(numberOf1BitsLoop(11));  // 3
    }
}
