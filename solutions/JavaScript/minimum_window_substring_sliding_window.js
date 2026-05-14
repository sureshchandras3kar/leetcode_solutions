/**
 * @param {string} s
 * @param {string} t
 * @return {string}
 */
function minWindowSlidingWindow(s, t) {
    if (!s || !t || s.length < t.length) {
        return "";
    }

    // Dictionary to store frequency of characters in t
    const dictT = {};
    for (const char of t) {
        dictT[char] = (dictT[char] || 0) + 1;
    }

    const required = Object.keys(dictT).length;
    let formed = 0;

    const windowCounts = {};

    // ans array: [window length, left, right]
    let ansLen = Infinity;
    let ansLeft = 0, ansRight = 0;

    let l = 0;

    for (let r = 0; r < s.length; r++) {
        const charR = s[r];
        windowCounts[charR] = (windowCounts[charR] || 0) + 1;

        if (dictT[charR] && windowCounts[charR] === dictT[charR]) {
            formed++;
        }

        while (l <= r && formed === required) {
            const charL = s[l];

            if (r - l + 1 < ansLen) {
                ansLen = r - l + 1;
                ansLeft = l;
                ansRight = r;
            }

            windowCounts[charL]--;
            if (dictT[charL] && windowCounts[charL] < dictT[charL]) {
                formed--;
            }

            l++;
        }
    }

    return ansLen === Infinity ? "" : s.substring(ansLeft, ansRight + 1);
}

console.log(minWindowSlidingWindow("ADOBECODEBANC", "ABC"));  // "BANC"
console.log(minWindowSlidingWindow("a", "a"));  // "a"
console.log(minWindowSlidingWindow("a", "aa"));  // ""
