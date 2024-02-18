function twoSumBruteForce(nums, target) {
    const n = nums.length;
    for (let i = 0; i < n; i++) {
        for (let j = i + 1; j < n; j++) {
            if (nums[i] + nums[j] === target) {
                return [i, j];
            }
        }
    }
    return []; // Return an empty array if no valid pair is found
}

const nums = [2, 7, 11, 15];
const target = 9;
const result = twoSumBruteForce(nums, target);
console.log(result);
