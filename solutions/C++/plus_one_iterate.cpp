#include <vector>
using namespace std;

vector<int> plusOneIterate(vector<int>& digits) {
    int carry = 1;
    for (int i = digits.size() - 1; i >= 0; i--) {
        digits[i] += carry;
        if (digits[i] < 10) {
            return digits;
        }
        digits[i] = 0;
    }

    digits.insert(digits.begin(), 1);
    return digits;
}

int main() {
    vector<int> digits = {1, 2, 3};
    vector<int> result = plusOneIterate(digits);
    return 0;
}
