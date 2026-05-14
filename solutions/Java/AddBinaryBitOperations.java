public class AddBinaryBitOperations {
    /*
    Bit operations approach - convert to long, add, convert back to binary.
    Time: O(max(a.length(), b.length()))
    Space: O(max(a.length(), b.length())) for result
    */
    public static String addBinaryBitOperations(String a, String b) {
        long numA = Long.parseLong(a, 2);  // Convert binary string to long
        long numB = Long.parseLong(b, 2);
        long total = numA + numB;
        return Long.toBinaryString(total);
    }

    public static void main(String[] args) {
        System.out.println(addBinaryBitOperations("11", "1"));      // "100"
        System.out.println(addBinaryBitOperations("1010", "1011")); // "10101"
        System.out.println(addBinaryBitOperations("0", "0"));       // "0"
    }
}
