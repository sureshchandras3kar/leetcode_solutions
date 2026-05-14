function findSubstring(s, words) {
    const result = [];

    if (!words || words.length === 0 || !s || s.length === 0) {
        return result;
    }

    const wordLen = words[0].length;
    const wordCount = words.length;
    const totalLen = wordLen * wordCount;

    // Count frequency of each word
    const wordFreq = {};
    for (const word of words) {
        wordFreq[word] = (wordFreq[word] || 0) + 1;
    }

    // Helper to compare two frequency maps
    const frequenciesMatch = (freq1, freq2) => {
        const keys1 = Object.keys(freq1).sort();
        const keys2 = Object.keys(freq2).sort();
        if (keys1.length !== keys2.length) return false;
        for (let i = 0; i < keys1.length; i++) {
            if (keys1[i] !== keys2[i] || freq1[keys1[i]] !== freq2[keys1[i]]) {
                return false;
            }
        }
        return true;
    };

    // For each possible starting position
    for (let i = 0; i <= s.length - totalLen; i++) {
        const windowFreq = {};

        // Extract and count words in this window
        for (let j = 0; j < wordCount; j++) {
            const word = s.substring(i + j * wordLen, i + (j + 1) * wordLen);
            windowFreq[word] = (windowFreq[word] || 0) + 1;
        }

        // Check if frequencies match
        if (frequenciesMatch(windowFreq, wordFreq)) {
            result.push(i);
        }
    }

    return result;
}

// Test cases
console.log(findSubstring("barfoothefoobarman", ["foo", "bar"]));          // [0, 9]
console.log(findSubstring("wordgoodgoodgoodbestword", ["word", "good", "best", "word"])); // []
console.log(findSubstring("barfoofoobarthefoobarman", ["bar", "foo", "the"])); // [6, 9, 12]
