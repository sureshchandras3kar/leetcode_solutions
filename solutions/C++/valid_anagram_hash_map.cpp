#include <string>
#include <unordered_map>
using namespace std;

class Solution {
public:
    bool isAnagram(string s, string t) {
        if (s.size() != t.size()) return false;
        unordered_map<char, int> count;
        for (char ch : s) count[ch]++;
        for (char ch : t) {
            if (!count.count(ch)) return false;
            count[ch]--;
            if (count[ch] == 0) count.erase(ch);
        }
        return count.empty();
    }
};
