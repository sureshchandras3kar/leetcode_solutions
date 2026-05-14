/**
 * @param {string} s
 * @param {string} t
 * @return {string}
 */
function minWindowBruteForce(s, t) {
    if (!s || !t || s.length < t.length) {
        return "";
    }

    const dictT = {};
    for (const char of t) {
        dictT[char] = (dictT[char] || 0) + 1;
    }

    let minLen = Infinity;
    let minStart = 0;

    // Check all possible substrings
    for (let i = 0; i < s.length; i++) {
        for (let j = i + 1; j <= s.length; j++) {
            const substring = s.substring(i, j);

            const substringCount = {};
            for (const char of substring) {
                substringCount[char] = (substringCount[char] || 0) + 1;
            }

            // Verify if this substring is valid
            let valid = true;
            for (const char in dictT) {
                if ((substringCount[char] || 0) < dictT[char]) {
                    valid = false;
                    break;
                }
            }

            // Update minimum if this is a valid and shorter substring
            if (valid && substring.length < minLen) {
                minLen = substring.length;
                minStart = i;
            }
        }
    }

    return minLen === Infinity ? "" : s.substring(minStart, minStart + minLen);
}

console.log(minWindowBruteForce("ADOBECODEBANC", "ABC"));  // "BANC"
console.log(minWindowBruteForce("a", "a"));  // "a"
console.log(minWindowBruteForce("a", "aa"));  // ""
