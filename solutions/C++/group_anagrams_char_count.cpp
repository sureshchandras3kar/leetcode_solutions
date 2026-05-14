#include <array>
#include <string>
#include <unordered_map>
#include <vector>
using namespace std;

class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        auto encode = [](const string& s) {
            array<int, 26> count{};
            for (char c : s) count[c - 'a']++;
            string key;
            for (int n : count) key += to_string(n) + ',';
            return key;
        };
        unordered_map<string, vector<string>> groups;
        for (const string& s : strs) {
            groups[encode(s)].push_back(s);
        }
        vector<vector<string>> result;
        for (auto& [k, v] : groups) {
            result.push_back(v);
        }
        return result;
    }
};
