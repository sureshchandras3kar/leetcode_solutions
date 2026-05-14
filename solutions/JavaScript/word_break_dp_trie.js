class TrieNode {
    constructor() {
        this.children = new Map();
        this.is_end = false;
    }
}

/**
 * Determine if string can be segmented using words from dictionary (Trie approach).
 *
 * Time Complexity: O(n*m)
 * Space Complexity: O(n + T)
 */
function wordBreakDpTrie(s, wordDict) {
    const root = new TrieNode();

    for (const word of wordDict) {
        let node = root;
        for (const ch of word) {
            if (!node.children.has(ch)) {
                node.children.set(ch, new TrieNode());
            }
            node = node.children.get(ch);
        }
        node.is_end = true;
    }

    const dp = new Array(s.length + 1).fill(false);
    dp[0] = true;

    for (let i = 1; i <= s.length; i++) {
        if (!dp[i]) {
            let node = root;
            for (let j = i - 1; j >= 0; j--) {
                const ch = s[j];
                if (!node.children.has(ch)) {
                    break;
                }
                node = node.children.get(ch);
                if (dp[j] && node.is_end) {
                    dp[i] = true;
                    break;
                }
            }
        }
    }

    return dp[s.length];
}

console.log(wordBreakDpTrie("leetcode", ["leet", "code"]));  // true
console.log(wordBreakDpTrie("applepenapple", ["apple", "pen"]));  // true
console.log(wordBreakDpTrie("catsandog", ["cat", "cats", "and", "sand", "dog"]));  // false
