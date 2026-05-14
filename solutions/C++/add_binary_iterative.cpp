#include <string>
using namespace std;

string addBinaryIterative(string a, string b) {
    string result = "";
    int carry = 0;
    int i = a.length() - 1;
    int j = b.length() - 1;

    while (i >= 0 || j >= 0 || carry) {
        int digitA = (i >= 0) ? (a[i] - '0') : 0;
        int digitB = (j >= 0) ? (b[j] - '0') : 0;
        int total = digitA + digitB + carry;
        result = char((total % 2) + '0') + result;
        carry = total / 2;
        i--;
        j--;
    }

    return result;
}

int main() {
    cout << addBinaryIterative("11", "1") << endl;  // "100"
    return 0;
}
