pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    if points.is_empty() { return 0; }
    points.sort_by_key(|p| p[1]);
    let mut arrows = 1;
    let mut last_end = points[0][1] as i64;
    for i in 1..points.len() {
        if (points[i][0] as i64) > last_end {
            arrows += 1;
            last_end = points[i][1] as i64;
        }
    }
    arrows
}
