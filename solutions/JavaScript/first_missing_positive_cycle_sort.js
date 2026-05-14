/**
 * @param {number[]} nums
 * @return {number}
 */
function firstMissingPositive(nums) {
    const n = nums.length;
    let i = 0;

    // Place each number at its correct index (value v at index v-1)
    while (i < n) {
        const j = nums[i] - 1;
        if (nums[i] >= 1 && nums[i] <= n && nums[j] !== nums[i]) {
            [nums[i], nums[j]] = [nums[j], nums[i]];
        } else {
            i++;
        }
    }

    // Find first index where value doesn't match expected
    for (let i = 0; i < n; i++) {
        if (nums[i] !== i + 1) {
            return i + 1;
        }
    }

    return n + 1;
}
