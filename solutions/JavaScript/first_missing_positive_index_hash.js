/**
 * @param {number[]} nums
 * @return {number}
 */
function firstMissingPositive(nums) {
    const n = nums.length;

    // Phase 1: replace non-positives and values > n with n+1
    for (let i = 0; i < n; i++) {
        if (nums[i] <= 0 || nums[i] > n) {
            nums[i] = n + 1;
        }
    }

    // Phase 2: mark presence using negative signs
    for (let i = 0; i < n; i++) {
        const v = Math.abs(nums[i]);
        if (v >= 1 && v <= n) {
            nums[v - 1] = -Math.abs(nums[v - 1]);
        }
    }

    // Phase 3: find first unmarked index
    for (let i = 0; i < n; i++) {
        if (nums[i] > 0) {
            return i + 1;
        }
    }

    return n + 1;
}
