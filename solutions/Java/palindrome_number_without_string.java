public class Palindrome {
    public static boolean isPalindrome(int x) {
        if (x < 0) {
            return false;
        }
        if (x == 0) {
            return true;
        }
        if (x % 10 == 0) {
            return false;
        }
        int numReversed = 0;
        int originalX = x;
        while (originalX > 0) {
            int lastDigit = originalX % 10;
            numReversed = numReversed * 10 + lastDigit;
            originalX /= 10;
        }
        return x == numReversed;
    }

    public static void main(String[] args) {
        System.out.println(isPalindrome(121));  // true
        System.out.println(isPalindrome(-121)); // false
        System.out.println(isPalindrome(10));   // false
    }
}
