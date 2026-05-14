fn three_sum_sort_two_pointer(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut result: Vec<Vec<i32>> = Vec::new();
    let n = nums.len();

    let mut i = 0;
    while i + 2 < n {
        if nums[i] > 0 {
            break;
        }
        if i > 0 && nums[i] == nums[i - 1] {
            i += 1;
            continue;
        }
        let mut left = i + 1;
        let mut right = n - 1;
        while left < right {
            let s = nums[i] + nums[left] + nums[right];
            if s == 0 {
                result.push(vec![nums[i], nums[left], nums[right]]);
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }
                left += 1;
                right -= 1;
            } else if s < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
        i += 1;
    }

    result
}

fn main() {
    let mut nums = vec![-1, 0, 1, 2, -1, -4];
    let result = three_sum_sort_two_pointer(&mut nums);
    println!("{:?}", result); // [[-1, -1, 2], [-1, 0, 1]]
}
