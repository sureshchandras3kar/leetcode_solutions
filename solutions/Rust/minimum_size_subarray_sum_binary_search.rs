fn min_sub_array_len_binary_search(target: i32, nums: &[i32]) -> i32 {
    let mut prefix = vec![0];
    for &num in nums {
        prefix.push(prefix[prefix.len() - 1] + num);
    }

    let mut min_length = i32::MAX;

    for right in 1..prefix.len() {
        let needed = prefix[right] - target;
        let left_pos = binary_search_rightmost(&prefix, needed, 0, right);
        let left = if left_pos > 0 { left_pos - 1 } else { 0 };

        if left < right {
            min_length = min_length.min((right - left) as i32);
        }
    }

    if min_length == i32::MAX { 0 } else { min_length }
}

fn binary_search_rightmost(arr: &[i32], target: i32, lo: usize, hi: usize) -> usize {
    let mut lo = lo;
    let mut hi = hi;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] <= target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

fn main() {
    let nums = vec![2, 3, 1, 2, 4, 3];
    let target = 7;
    let result = min_sub_array_len_binary_search(target, &nums);
    println!("{}", result);  // 2
}
