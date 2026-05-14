#include <iostream>
#include <vector>
#include <string>
#include <algorithm>

std::vector<int> findSubstringBruteForce(
    const std::string& s,
    std::vector<std::string> words) {
    if (words.empty() || s.empty()) {
        return {};
    }

    int wordLen = words[0].length();
    int wordCount = words.size();
    int totalLen = wordLen * wordCount;

    std::vector<int> result;

    // For each possible starting position in the string
    for (int i = 0; i <= (int)s.length() - totalLen; i++) {
        // Extract the substring of exact length
        std::string substring = s.substr(i, totalLen);

        // Check if this substring can be formed by concatenating all words
        std::vector<std::string> tempWords = words;
        bool valid = true;

        for (int j = 0; j < wordCount; j++) {
            std::string word = substring.substr(j * wordLen, wordLen);
            auto it = std::find(tempWords.begin(), tempWords.end(), word);
            if (it != tempWords.end()) {
                tempWords.erase(it);
            } else {
                valid = false;
                break;
            }
        }

        if (valid && tempWords.empty()) {
            result.push_back(i);
        }
    }

    return result;
}

int main() {
    // Example 1
    std::string s1 = "barfoothefoobarman";
    std::vector<std::string> words1 = {"foo", "bar"};
    std::vector<int> result1 = findSubstringBruteForce(s1, words1);
    std::cout << "Example 1: ";
    for (int idx : result1) {
        std::cout << idx << " ";
    }
    std::cout << std::endl; // [0, 9]

    // Example 2
    std::string s2 = "wordgoodgoodgoodbestword";
    std::vector<std::string> words2 = {"word", "good", "best", "word"};
    std::vector<int> result2 = findSubstringBruteForce(s2, words2);
    std::cout << "Example 2: ";
    for (int idx : result2) {
        std::cout << idx << " ";
    }
    std::cout << std::endl; // []

    // Example 3
    std::string s3 = "barfoofoobarthefoobarman";
    std::vector<std::string> words3 = {"bar", "foo", "the"};
    std::vector<int> result3 = findSubstringBruteForce(s3, words3);
    std::cout << "Example 3: ";
    for (int idx : result3) {
        std::cout << idx << " ";
    }
    std::cout << std::endl; // [6, 9, 12]

    return 0;
}
