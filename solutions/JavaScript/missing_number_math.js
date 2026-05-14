function missingNumber(nums) {
    const n = nums.length;
    const expected = n * (n + 1) / 2;
    const actual = nums.reduce((sum, n) => sum + n, 0);
    return expected - actual;
}
