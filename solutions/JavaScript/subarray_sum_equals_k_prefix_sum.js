function subarraySum(nums, k) {
    const count = new Map([[0, 1]]);
    let prefix = 0, result = 0;
    for (const num of nums) {
        prefix += num;
        result += count.get(prefix - k) ?? 0;
        count.set(prefix, (count.get(prefix) ?? 0) + 1);
    }
    return result;
}
