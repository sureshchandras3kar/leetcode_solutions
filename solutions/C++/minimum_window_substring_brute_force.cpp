#include <iostream>
#include <string>
#include <unordered_map>
#include <climits>

std::string minWindowBruteForce(std::string s, std::string t) {
    if (s.empty() || t.empty() || s.length() < t.length()) {
        return "";
    }

    std::unordered_map<char, int> dict_t;
    for (char c : t) {
        dict_t[c]++;
    }

    int min_len = INT_MAX;
    int min_start = 0;

    // Check all possible substrings
    for (int i = 0; i < (int)s.length(); i++) {
        for (int j = i + 1; j <= (int)s.length(); j++) {
            std::string substring = s.substr(i, j - i);

            std::unordered_map<char, int> substring_count;
            for (char c : substring) {
                substring_count[c]++;
            }

            // Verify if this substring is valid
            bool valid = true;
            for (auto& pair : dict_t) {
                if (substring_count[pair.first] < pair.second) {
                    valid = false;
                    break;
                }
            }

            // Update minimum if this is a valid and shorter substring
            if (valid && (int)substring.length() < min_len) {
                min_len = substring.length();
                min_start = i;
            }
        }
    }

    return min_len == INT_MAX ? "" : s.substr(min_start, min_len);
}

int main() {
    std::cout << minWindowBruteForce("ADOBECODEBANC", "ABC") << std::endl;  // "BANC"
    std::cout << minWindowBruteForce("a", "a") << std::endl;  // "a"
    std::cout << minWindowBruteForce("a", "aa") << std::endl;  // ""
    return 0;
}
