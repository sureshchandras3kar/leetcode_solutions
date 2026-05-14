/**
 * @param {number[]} nums
 * @return {number[]}
 */
function productExceptSelf(nums) {
    const n = nums.length;
    const prefix = new Array(n).fill(1);
    const suffix = new Array(n).fill(1);

    for (let i = 1; i < n; i++)
        prefix[i] = prefix[i - 1] * nums[i - 1];

    for (let i = n - 2; i >= 0; i--)
        suffix[i] = suffix[i + 1] * nums[i + 1];

    return Array.from({ length: n }, (_, i) => prefix[i] * suffix[i]);
}
