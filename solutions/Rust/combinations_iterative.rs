pub fn combinations_iterative(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut combo: Vec<i32> = (1..=k).collect();

    loop {
        result.push(combo.clone());

        // Find the rightmost number that can be incremented
        let mut i = (k - 1) as usize;
        loop {
            if combo[i] == n - k + (i as i32) + 1 {
                if i == 0 {
                    return result;
                }
                i -= 1;
            } else {
                break;
            }
        }

        // Increment and reset
        combo[i] += 1;
        for j in (i + 1)..(k as usize) {
            combo[j] = combo[j - 1] + 1;
        }
    }
}

fn main() {
    let result = combinations_iterative(4, 2);
    println!("{:?}", result);
    // Output: [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
}
