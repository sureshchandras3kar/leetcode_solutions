pub fn insert(intervals: Vec<Vec<i32>>, newInterval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut i = 0;
    let mut new_start = newInterval[0];
    let mut new_end = newInterval[1];
    while i < intervals.len() && intervals[i][1] < new_start {
        result.push(intervals[i].clone());
        i += 1;
    }
    while i < intervals.len() && intervals[i][0] <= new_end {
        new_start = new_start.min(intervals[i][0]);
        new_end = new_end.max(intervals[i][1]);
        i += 1;
    }
    result.push(vec![new_start, new_end]);
    while i < intervals.len() {
        result.push(intervals[i].clone());
        i += 1;
    }
    result
}
