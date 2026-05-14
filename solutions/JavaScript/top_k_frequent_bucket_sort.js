/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
function topKFrequent(nums, k) {
    const freq = new Map();
    for (const num of nums) {
        freq.set(num, (freq.get(num) ?? 0) + 1);
    }

    const n = nums.length;
    const buckets = Array.from({ length: n + 1 }, () => []);
    for (const [num, count] of freq) {
        buckets[count].push(num);
    }

    const result = [];
    for (let i = n; i >= 1 && result.length < k; i--) {
        for (const num of buckets[i]) {
            result.push(num);
            if (result.length === k) return result;
        }
    }
    return result;
}
