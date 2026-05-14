public class AddBinaryIterative {
    public static String addBinaryIterative(String a, String b) {
        StringBuilder result = new StringBuilder();
        int carry = 0;
        int i = a.length() - 1;
        int j = b.length() - 1;

        while (i >= 0 || j >= 0 || carry > 0) {
            int digitA = (i >= 0) ? (a.charAt(i) - '0') : 0;
            int digitB = (j >= 0) ? (b.charAt(j) - '0') : 0;
            int total = digitA + digitB + carry;
            result.append(total % 2);
            carry = total / 2;
            i--;
            j--;
        }

        return result.reverse().toString();
    }

    public static void main(String[] args) {
        System.out.println(addBinaryIterative("11", "1"));  // "100"
    }
}
