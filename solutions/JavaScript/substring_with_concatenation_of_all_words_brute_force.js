function findSubstring(s, words) {
    const result = [];

    if (!words || words.length === 0 || !s || s.length === 0) {
        return result;
    }

    const wordLen = words[0].length;
    const wordCount = words.length;
    const totalLen = wordLen * wordCount;

    // For each possible starting position in the string
    for (let i = 0; i <= s.length - totalLen; i++) {
        // Extract the substring of exact length
        const substring = s.substring(i, i + totalLen);

        // Check if this substring can be formed by concatenating all words
        const tempWords = [...words];
        let valid = true;

        for (let j = 0; j < wordCount; j++) {
            const word = substring.substring(j * wordLen, (j + 1) * wordLen);
            const idx = tempWords.indexOf(word);
            if (idx !== -1) {
                tempWords.splice(idx, 1);
            } else {
                valid = false;
                break;
            }
        }

        if (valid && tempWords.length === 0) {
            result.push(i);
        }
    }

    return result;
}

// Test cases
console.log(findSubstring("barfoothefoobarman", ["foo", "bar"]));          // [0, 9]
console.log(findSubstring("wordgoodgoodgoodbestword", ["word", "good", "best", "word"])); // []
console.log(findSubstring("barfoofoobarthefoobarman", ["bar", "foo", "the"])); // [6, 9, 12]
