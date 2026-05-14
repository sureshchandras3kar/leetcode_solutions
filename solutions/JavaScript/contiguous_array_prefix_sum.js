function findMaxLength(nums) {
    const seen = new Map([[0, -1]]);
    let prefix = 0, best = 0;
    for (let i = 0; i < nums.length; i++) {
        prefix += nums[i] === 1 ? 1 : -1;
        if (seen.has(prefix)) {
            best = Math.max(best, i - seen.get(prefix));
        } else {
            seen.set(prefix, i);
        }
    }
    return best;
}
