function longestConsecutive(nums) {
    if (nums.length === 0) return 0;
    nums.sort((a, b) => a - b);
    let best = 1, streak = 1;
    for (let i = 1; i < nums.length; i++) {
        if (nums[i] === nums[i - 1]) continue;  // skip duplicate
        if (nums[i] === nums[i - 1] + 1) {
            streak++;
            best = Math.max(best, streak);
        } else {
            streak = 1;
        }
    }
    return best;
}
