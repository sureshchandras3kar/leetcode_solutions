function singleNumberIIBitCount(nums) {
    const bitCounts = new Array(32).fill(0);
    for (const num of nums) {
        for (let i = 0; i < 32; i++) {
            if (num & (1 << i)) {
                bitCounts[i]++;
            }
        }
    }

    let result = 0;
    for (let i = 0; i < 32; i++) {
        if (bitCounts[i] % 3) {
            result |= (1 << i);
        }
    }

    if (result >= Math.pow(2, 31)) {
        result -= Math.pow(2, 32);
    }
    return result;
}

console.log(singleNumberIIBitCount([2, 2, 3, 2]));  // 3
