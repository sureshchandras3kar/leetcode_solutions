function twoSumHashMap(nums, target) {
    const seen = {};
    for (let i = 0; i < nums.length; i++) {
        const complement = target - nums[i];
        if (complement in seen) {
            return [seen[complement], i];
        }
        seen[nums[i]] = i;
    }
    return [];
}

const nums = [2, 7, 11, 15];
const target = 9;
const result = twoSumHashMap(nums, target);
console.log(result);
