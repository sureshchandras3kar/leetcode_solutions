fn min_sub_array_len_sliding_window(target: i32, nums: &[i32]) -> i32 {
    let mut left = 0;
    let mut current_sum = 0;
    let mut min_length = i32::MAX;

    for (right, &num) in nums.iter().enumerate() {
        current_sum += num;

        while current_sum >= target {
            min_length = min_length.min((right - left + 1) as i32);
            current_sum -= nums[left];
            left += 1;
        }
    }

    if min_length == i32::MAX { 0 } else { min_length }
}

fn main() {
    let nums = vec![2, 3, 1, 2, 4, 3];
    let target = 7;
    let result = min_sub_array_len_sliding_window(target, &nums);
    println!("{}", result);  // 2
}
