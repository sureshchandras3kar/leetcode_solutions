/**
 * Generate all combinations of k numbers from 1 to n using iterative approach.
 * Time: O(C(n,k) * k), Space: O(C(n,k) * k) for result
 * @param {number} n
 * @param {number} k
 * @return {number[][]}
 */
function combinationsIterative(n, k) {
    const result = [];
    const combo = Array.from({ length: k }, (_, i) => i + 1);

    while (true) {
        result.push([...combo]);

        // Find the rightmost number that can be incremented
        let i = k - 1;
        while (i >= 0 && combo[i] === n - k + i + 1) {
            i--;
        }

        if (i < 0) break;

        // Increment and reset
        combo[i]++;
        for (let j = i + 1; j < k; j++) {
            combo[j] = combo[j - 1] + 1;
        }
    }

    return result;
}

console.log(combinationsIterative(4, 2));
// Output: [[1,2], [1,3], [1,4], [2,3], [2,4], [3,4]]
