pub fn insert(intervals: Vec<Vec<i32>>, mut newInterval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    for interval in intervals {
        if interval[1] < newInterval[0] {
            result.push(interval);
        } else if interval[0] > newInterval[1] {
            result.push(newInterval);
            newInterval = interval;
        } else {
            newInterval[0] = newInterval[0].min(interval[0]);
            newInterval[1] = newInterval[1].max(interval[1]);
        }
    }
    result.push(newInterval);
    result
}
