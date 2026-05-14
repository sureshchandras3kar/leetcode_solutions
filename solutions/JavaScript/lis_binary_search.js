/**
 * Find length of longest increasing subsequence using binary search O(n log n).
 *
 * Time Complexity: O(n log n)
 * Space Complexity: O(n)
 */
function lisBinarySearch(nums) {
    if (nums.length === 0) return 0;

    const tails = [];

    for (const num of nums) {
        let left = 0, right = tails.length;
        while (left < right) {
            const mid = Math.floor((left + right) / 2);
            if (tails[mid] < num) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if (left === tails.length) {
            tails.push(num);
        } else {
            tails[left] = num;
        }
    }

    return tails.length;
}

console.log(lisBinarySearch([10, 9, 2, 5, 3, 7, 101, 18]));  // 4
