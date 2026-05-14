#include <iostream>
#include <unordered_set>
#include <string>
#include <algorithm>

int longest_substring_without_repeating_characters_brute_force(std::string s) {
    int max_length = 0;

    for (int i = 0; i < (int)s.length(); i++) {
        std::unordered_set<char> char_set;
        for (int j = i; j < (int)s.length(); j++) {
            if (char_set.count(s[j])) {
                break;
            }
            char_set.insert(s[j]);
            max_length = std::max(max_length, j - i + 1);
        }
    }

    return max_length;
}

int main() {
    std::string s = "abcabcbb";
    std::cout << longest_substring_without_repeating_characters_brute_force(s) << std::endl;
    return 0;
}
