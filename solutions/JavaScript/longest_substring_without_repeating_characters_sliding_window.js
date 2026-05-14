function longestSubstringWithoutRepeatingCharactersSlidingWindow(s) {
    const charIndex = {};
    let maxLength = 0;
    let left = 0;

    for (let right = 0; right < s.length; right++) {
        if (s[right] in charIndex && charIndex[s[right]] >= left) {
            left = charIndex[s[right]] + 1;
        }
        charIndex[s[right]] = right;
        maxLength = Math.max(maxLength, right - left + 1);
    }

    return maxLength;
}

const s = "abcabcbb";
const result = longestSubstringWithoutRepeatingCharactersSlidingWindow(s);
console.log(result);
