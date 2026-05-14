#include <string>
#include <iostream>

using namespace std;

string addBinaryStringSimulation(string a, string b) {
    /*
    String simulation approach - simulate binary addition from right to left.
    Time: O(max(a.size(), b.size()))
    Space: O(max(a.size(), b.size())) for result
    */
    string result = "";
    int carry = 0;
    int i = a.size() - 1;
    int j = b.size() - 1;

    while (i >= 0 || j >= 0 || carry) {
        int digit_a = (i >= 0) ? (a[i] - '0') : 0;
        int digit_b = (j >= 0) ? (b[j] - '0') : 0;

        int total = digit_a + digit_b + carry;
        result = char('0' + (total % 2)) + result;
        carry = total / 2;

        i--;
        j--;
    }

    return result;
}

int main() {
    cout << addBinaryStringSimulation("11", "1") << endl;      // "100"
    cout << addBinaryStringSimulation("1010", "1011") << endl; // "10101"
    cout << addBinaryStringSimulation("0", "0") << endl;       // "0"
    return 0;
}
