#include <iostream>
#include <string>
#include <unordered_map>
#include <climits>

std::string minWindowSlidingWindow(std::string s, std::string t) {
    if (s.empty() || t.empty() || s.length() < t.length()) {
        return "";
    }

    std::unordered_map<char, int> dict_t;
    for (char c : t) {
        dict_t[c]++;
    }

    int required = dict_t.size();
    int formed = 0;

    std::unordered_map<char, int> window_counts;

    // ans array: {window length, left, right}
    int ans_len = INT_MAX;
    int ans_left = 0, ans_right = 0;

    int l = 0;

    for (int r = 0; r < (int)s.length(); r++) {
        char char_r = s[r];
        window_counts[char_r]++;

        if (dict_t.count(char_r) && window_counts[char_r] == dict_t[char_r]) {
            formed++;
        }

        while (l <= r && formed == required) {
            char char_l = s[l];

            if (r - l + 1 < ans_len) {
                ans_len = r - l + 1;
                ans_left = l;
                ans_right = r;
            }

            window_counts[char_l]--;
            if (dict_t.count(char_l) && window_counts[char_l] < dict_t[char_l]) {
                formed--;
            }

            l++;
        }
    }

    return ans_len == INT_MAX ? "" : s.substr(ans_left, ans_len);
}

int main() {
    std::cout << minWindowSlidingWindow("ADOBECODEBANC", "ABC") << std::endl;  // "BANC"
    std::cout << minWindowSlidingWindow("a", "a") << std::endl;  // "a"
    std::cout << minWindowSlidingWindow("a", "aa") << std::endl;  // ""
    return 0;
}
