function threeSumHashSet(nums) {
    nums.sort((a, b) => a - b);
    const result = [];
    const n = nums.length;

    for (let i = 0; i < n - 2; i++) {
        if (i > 0 && nums[i] === nums[i - 1]) continue;
        const seen = new Set();
        for (let j = i + 1; j < n; j++) {
            const need = -(nums[i] + nums[j]);
            if (seen.has(need)) {
                const triplet = [nums[i], need, nums[j]];
                const key = triplet.join(',');
                if (!result.some(t => t.join(',') === key)) {
                    result.push(triplet);
                }
            }
            seen.add(nums[j]);
        }
    }

    return result;
}

console.log(threeSumHashSet([-1, 0, 1, 2, -1, -4])); // [[-1,-1,2],[-1,0,1]]
console.log(threeSumHashSet([0, 1, 1]));               // []
console.log(threeSumHashSet([0, 0, 0]));               // [[0,0,0]]
