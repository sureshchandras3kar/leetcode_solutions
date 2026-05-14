fn two_sum_two_pointer(nums: &[i32], target: i32) -> Vec<i32> {
    let mut sorted = nums.to_vec();
    sorted.sort();
    let (mut left, mut right) = (0, sorted.len() - 1);
    while left < right {
        let current_sum = sorted[left] + sorted[right];
        if current_sum == target {
            return vec![sorted[left], sorted[right]];
        } else if current_sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum_two_pointer(&nums, target);
    println!("{:?}", result);
}
