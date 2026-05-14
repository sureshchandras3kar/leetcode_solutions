#include <cstdint>
using namespace std;

int numberOf1BitsLoop(uint32_t n) {
    int count = 0;
    while (n) {
        count += n & 1;
        n >>= 1;
    }
    return count;
}

int main() {
    cout << numberOf1BitsLoop(11) << endl;  // 3
    return 0;
}
