/**
 * Determine if string can be segmented using words from dictionary.
 *
 * Time Complexity: O(n²)
 * Space Complexity: O(n + m)
 */
function wordBreakDpSet(s, wordDict) {
    const wordSet = new Set(wordDict);
    const dp = new Array(s.length + 1).fill(false);
    dp[0] = true;

    for (let i = 1; i <= s.length; i++) {
        for (let j = 0; j < i; j++) {
            if (dp[j] && wordSet.has(s.substring(j, i))) {
                dp[i] = true;
                break;
            }
        }
    }

    return dp[s.length];
}

console.log(wordBreakDpSet("leetcode", ["leet", "code"]));  // true
console.log(wordBreakDpSet("applepenapple", ["apple", "pen"]));  // true
console.log(wordBreakDpSet("catsandog", ["cat", "cats", "and", "sand", "dog"]));  // false
