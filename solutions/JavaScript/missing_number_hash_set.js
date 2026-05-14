function missingNumber(nums) {
    const seen = new Set(nums);
    for (let value = 0; value <= nums.length; value++) {
        if (!seen.has(value)) return value;
    }
    return -1;
}
