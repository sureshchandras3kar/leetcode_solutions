function subarraySum(nums, k) {
    let result = 0;
    for (let i = 0; i < nums.length; i++) {
        let total = 0;
        for (let j = i; j < nums.length; j++) {
            total += nums[j];
            if (total === k) result++;
        }
    }
    return result;
}
