use std::collections::HashSet;

fn three_sum_hash_set(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut result: Vec<Vec<i32>> = Vec::new();
    let n = nums.len();

    for i in 0..n.saturating_sub(2) {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let mut seen: HashSet<i32> = HashSet::new();
        for j in (i + 1)..n {
            let need = -(nums[i] + nums[j]);
            if seen.contains(&need) {
                let triplet = vec![nums[i], need, nums[j]];
                if !result.contains(&triplet) {
                    result.push(triplet);
                }
            }
            seen.insert(nums[j]);
        }
    }

    result
}

fn main() {
    let mut nums = vec![-1, 0, 1, 2, -1, -4];
    let result = three_sum_hash_set(&mut nums);
    println!("{:?}", result); // [[-1, -1, 2], [-1, 0, 1]]
}
