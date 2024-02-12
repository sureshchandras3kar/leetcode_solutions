#include <iostream>

bool isPalindrome(int x) {
    if (x < 0) {
        return false;
    }
    if (x == 0) {
        return true;
    }
    if (x % 10 == 0) {
        return false;
    }
    int numReversed = 0;
    int originalX = x;
    while (originalX > 0) {
        int lastDigit = originalX % 10;
        numReversed = numReversed * 10 + lastDigit;
        originalX /= 10;
    }
    return x == numReversed;
}

int main() {
    std::cout << std::boolalpha;
    std::cout << isPalindrome(121) << std::endl;  // true
    std::cout << isPalindrome(-121) << std::endl; // false
    std::cout << isPalindrome(10) << std::endl;   // false
    return 0;
}
