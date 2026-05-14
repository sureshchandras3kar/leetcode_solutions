function merge(intervals) {
    const pq = intervals.sort((a, b) => a[0] - b[0]);
    const merged = [pq.shift()];
    for (let interval of pq) {
        if (interval[0] <= merged[merged.length - 1][1]) {
            merged[merged.length - 1][1] = Math.max(merged[merged.length - 1][1], interval[1]);
        } else {
            merged.push(interval);
        }
    }
    return merged;
}
module.exports = merge;
