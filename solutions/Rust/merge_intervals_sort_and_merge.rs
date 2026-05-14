pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() { return vec![]; }
    intervals.sort_by_key(|a| a[0]);
    let mut merged = vec![intervals[0].clone()];
    for interval in &intervals[1..] {
        let last = merged.last_mut().unwrap();
        if interval[0] <= last[1] {
            last[1] = last[1].max(interval[1]);
        } else {
            merged.push(interval.clone());
        }
    }
    merged
}
