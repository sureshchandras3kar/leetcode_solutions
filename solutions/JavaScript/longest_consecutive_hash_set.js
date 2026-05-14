function longestConsecutive(nums) {
    const numSet = new Set(nums);
    let best = 0;
    for (const n of numSet) {
        if (!numSet.has(n - 1)) {  // start of a sequence
            let length = 1;
            while (numSet.has(n + length)) {
                length++;
            }
            best = Math.max(best, length);
        }
    }
    return best;
}
