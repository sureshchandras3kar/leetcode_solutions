/**
 * Generate all letter combinations using backtracking (recursive).
 * Time: O(4^n), Space: O(4^n) for result
 * @param {string} digits
 * @return {string[]}
 */
function letterCombinationsBacktracking(digits) {
    if (!digits) return [];

    const phoneMap = {
        "2": "abc",
        "3": "def",
        "4": "ghi",
        "5": "jkl",
        "6": "mno",
        "7": "pqrs",
        "8": "tuv",
        "9": "wxyz"
    };

    const result = [];

    function backtrack(index, currentCombination) {
        // Base case: we've processed all digits
        if (index === digits.length) {
            result.push(currentCombination);
            return;
        }

        // Get the letters that the current digit maps to
        const currentDigit = digits[index];
        const letters = phoneMap[currentDigit];

        // Try each letter
        for (const letter of letters) {
            backtrack(index + 1, currentCombination + letter);
        }
    }

    backtrack(0, "");
    return result;
}

console.log(letterCombinationsBacktracking("23"));
// Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
