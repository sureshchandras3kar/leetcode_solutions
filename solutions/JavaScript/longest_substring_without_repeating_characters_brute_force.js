function longestSubstringWithoutRepeatingCharactersBruteForce(s) {
    let maxLength = 0;

    for (let i = 0; i < s.length; i++) {
        const charSet = new Set();
        for (let j = i; j < s.length; j++) {
            if (charSet.has(s[j])) {
                break;
            }
            charSet.add(s[j]);
            maxLength = Math.max(maxLength, j - i + 1);
        }
    }

    return maxLength;
}

const s = "abcabcbb";
const result = longestSubstringWithoutRepeatingCharactersBruteForce(s);
console.log(result);
