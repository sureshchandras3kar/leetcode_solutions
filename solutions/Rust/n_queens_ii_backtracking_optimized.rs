use std::collections::HashSet;

pub fn total_n_queens(n: i32) -> i32 {
    let n = n as usize;
    fn backtrack(row: usize, cols: &mut HashSet<usize>, diag1: &mut HashSet<i32>, diag2: &mut HashSet<i32>, n: usize) -> i32 {
        if row == n { return 1; }
        let mut count = 0;
        for col in 0..n {
            let d1 = row as i32 - col as i32;
            let d2 = row as i32 + col as i32;
            if !cols.contains(&col) && !diag1.contains(&d1) && !diag2.contains(&d2) {
                cols.insert(col);
                diag1.insert(d1);
                diag2.insert(d2);
                count += backtrack(row + 1, cols, diag1, diag2, n);
                cols.remove(&col);
                diag1.remove(&d1);
                diag2.remove(&d2);
            }
        }
        count
    }
    backtrack(0, &mut HashSet::new(), &mut HashSet::new(), &mut HashSet::new(), n)
}
