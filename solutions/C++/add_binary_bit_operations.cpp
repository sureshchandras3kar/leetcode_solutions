#include <string>
#include <iostream>

using namespace std;

string addBinaryBitOperations(string a, string b) {
    /*
    Bit operations approach - convert to long, add, convert back to binary.
    Time: O(max(a.size(), b.size()))
    Space: O(max(a.size(), b.size())) for result
    */
    long num_a = stoll(a, nullptr, 2);  // Convert binary string to long
    long num_b = stoll(b, nullptr, 2);
    long total = num_a + num_b;

    // Convert back to binary string
    if (total == 0) return "0";
    string result = "";
    while (total > 0) {
        result = char('0' + (total % 2)) + result;
        total /= 2;
    }
    return result;
}

int main() {
    cout << addBinaryBitOperations("11", "1") << endl;      // "100"
    cout << addBinaryBitOperations("1010", "1011") << endl; // "10101"
    cout << addBinaryBitOperations("0", "0") << endl;       // "0"
    return 0;
}
