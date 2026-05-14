#include <iostream>
#include <unordered_map>
#include <string>
#include <algorithm>

int longest_substring_without_repeating_characters_sliding_window(std::string s) {
    std::unordered_map<char, int> char_index;
    int max_length = 0;
    int left = 0;

    for (int right = 0; right < (int)s.length(); right++) {
        if (char_index.count(s[right]) && char_index[s[right]] >= left) {
            left = char_index[s[right]] + 1;
        }
        char_index[s[right]] = right;
        max_length = std::max(max_length, right - left + 1);
    }

    return max_length;
}

int main() {
    std::string s = "abcabcbb";
    std::cout << longest_substring_without_repeating_characters_sliding_window(s) << std::endl;
    return 0;
}
