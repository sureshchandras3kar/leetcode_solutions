#include <cstdint>
using namespace std;

uint32_t reverseBitsIterate(uint32_t n) {
    uint32_t result = 0;
    for (int i = 0; i < 32; i++) {
        result = (result << 1) | (n & 1);
        n >>= 1;
    }
    return result;
}

int main() {
    cout << reverseBitsIterate(43261596) << endl;  // 964176192
    return 0;
}
