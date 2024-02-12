#include <iostream>
#include <string>

bool isPalindrome(int x) {
    std::string strX = std::to_string(x);
    for (size_t i = 0; i < strX.length() / 2; i++) {
        if (strX[i] != strX[strX.length() - i - 1]) {
            return false;
        }
    }
    return true;
}

int main() {
    std::cout << std::boolalpha;
    std::cout << isPalindrome(121) << std::endl;  // true
    std::cout << isPalindrome(-121) << std::endl; // false
    std::cout << isPalindrome(10) << std::endl;   // false
    return 0;
}
