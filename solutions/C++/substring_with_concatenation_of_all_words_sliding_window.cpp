#include <iostream>
#include <vector>
#include <string>
#include <unordered_map>

std::vector<int> findSubstringWindowApproach(
    const std::string& s,
    const std::vector<std::string>& words) {
    if (words.empty() || s.empty()) {
        return {};
    }

    int wordLen = words[0].length();
    int wordCount = words.size();
    int totalLen = wordLen * wordCount;

    // Count frequency of each word
    std::unordered_map<std::string, int> wordFreq;
    for (const auto& word : words) {
        wordFreq[word]++;
    }

    std::vector<int> result;

    // For each possible starting position
    for (int i = 0; i <= (int)s.length() - totalLen; i++) {
        std::unordered_map<std::string, int> windowFreq;

        // Extract and count words in this window
        for (int j = 0; j < wordCount; j++) {
            std::string word = s.substr(i + j * wordLen, wordLen);
            windowFreq[word]++;
        }

        // Check if frequencies match
        if (windowFreq == wordFreq) {
            result.push_back(i);
        }
    }

    return result;
}

int main() {
    // Example 1
    std::string s1 = "barfoothefoobarman";
    std::vector<std::string> words1 = {"foo", "bar"};
    std::vector<int> result1 = findSubstringWindowApproach(s1, words1);
    std::cout << "Example 1: ";
    for (int idx : result1) {
        std::cout << idx << " ";
    }
    std::cout << std::endl; // [0, 9]

    // Example 2
    std::string s2 = "wordgoodgoodgoodbestword";
    std::vector<std::string> words2 = {"word", "good", "best", "word"};
    std::vector<int> result2 = findSubstringWindowApproach(s2, words2);
    std::cout << "Example 2: ";
    for (int idx : result2) {
        std::cout << idx << " ";
    }
    std::cout << std::endl; // []

    // Example 3
    std::string s3 = "barfoofoobarthefoobarman";
    std::vector<std::string> words3 = {"bar", "foo", "the"};
    std::vector<int> result3 = findSubstringWindowApproach(s3, words3);
    std::cout << "Example 3: ";
    for (int idx : result3) {
        std::cout << idx << " ";
    }
    std::cout << std::endl; // [6, 9, 12]

    return 0;
}
