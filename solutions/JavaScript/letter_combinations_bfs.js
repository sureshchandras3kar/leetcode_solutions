/**
 * Generate all letter combinations using BFS (iterative).
 * Time: O(4^n), Space: O(4^n) for result
 * @param {string} digits
 * @return {string[]}
 */
function letterCombinationsBFS(digits) {
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

    let queue = [""];

    for (const digit of digits) {
        const newQueue = [];
        const letters = phoneMap[digit];

        // Process all current combinations
        for (const current of queue) {
            // Create a new combination for each letter
            for (const letter of letters) {
                newQueue.push(current + letter);
            }
        }

        queue = newQueue;
    }

    return queue;
}

console.log(letterCombinationsBFS("23"));
// Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
