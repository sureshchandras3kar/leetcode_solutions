public class AddBinaryStringSimulation {
    /*
    String simulation approach - simulate binary addition from right to left.
    Time: O(max(a.length(), b.length()))
    Space: O(max(a.length(), b.length())) for result
    */
    public static String addBinaryStringSimulation(String a, String b) {
        StringBuilder result = new StringBuilder();
        int carry = 0;
        int i = a.length() - 1;
        int j = b.length() - 1;

        while (i >= 0 || j >= 0 || carry != 0) {
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
        System.out.println(addBinaryStringSimulation("11", "1"));      // "100"
        System.out.println(addBinaryStringSimulation("1010", "1011")); // "10101"
        System.out.println(addBinaryStringSimulation("0", "0"));       // "0"
    }
}
