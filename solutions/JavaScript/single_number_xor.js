function singleNumberXor(nums) {
    let result = 0;
    for (const num of nums) {
        result ^= num;
    }
    return result;
}

console.log(singleNumberXor([2, 2, 1]));  // 1
console.log(singleNumberXor([4, 1, 2, 1, 2]));  // 4
