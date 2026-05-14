public class Palindrome {
    public static boolean isPalindrome(int x) {
        String strX = Integer.toString(x);
        int n = strX.length();
        for (int i = 0; i < n / 2; i++) {
            if (strX.charAt(i) != strX.charAt(n - i - 1)) {
                return false;
            }
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(isPalindrome(121));  // true
        System.out.println(isPalindrome(-121)); // false
        System.out.println(isPalindrome(10));   // false
    }
}
