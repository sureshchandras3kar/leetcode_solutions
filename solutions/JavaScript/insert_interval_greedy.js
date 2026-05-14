function insert(intervals, newInterval) {
    const result = [];
    let i = 0, [new_start, new_end] = newInterval;
    while (i < intervals.length && intervals[i][1] < new_start) {
        result.push(intervals[i++]);
    }
    while (i < intervals.length && intervals[i][0] <= new_end) {
        new_start = Math.min(new_start, intervals[i][0]);
        new_end = Math.max(new_end, intervals[i][1]);
        i++;
    }
    result.push([new_start, new_end]);
    while (i < intervals.length) {
        result.push(intervals[i++]);
    }
    return result;
}
