fn two_sum_brute_force(nums: &[i32], target: i32) -> Vec<usize> {
    let n = nums.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if nums[i] + nums[j] == target {
                return vec![i, j];
            }
        }
    }
    vec![] // Return an empty vector if no valid pair is found
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum_brute_force(&nums, target);
    println!("{:?}", result);
}
