function majorityElement(nums) {
    const counts = new Map();
    const threshold = Math.floor(nums.length / 2);
    for (const num of nums) {
        const next = (counts.get(num) || 0) + 1;
        counts.set(num, next);
        if (next > threshold) return num;
    }
    return nums[0];
}
