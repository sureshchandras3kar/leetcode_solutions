/*
Two Pointers with Counter Approach
Allow each element to appear at most twice using explicit count tracking.

Time Complexity: O(n)
Space Complexity: O(1)
*/
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut k = 1;
    let mut count = 1;
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            // New element encountered, reset counter
            count = 1;
            nums[k] = nums[i];
            k += 1;
        } else if count < 2 {
            // Same element but less than 2 occurrences, allow it
            count += 1;
            nums[k] = nums[i];
            k += 1;
        }
        // else: count == 2, skip this duplicate
    }

    k as i32
}

fn main() {
    let mut nums1 = vec![1, 1, 1, 2, 2, 3];
    println!("{}", remove_duplicates(&mut nums1));  // 5, nums = [1, 1, 2, 2, 3, _]

    let mut nums2 = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    println!("{}", remove_duplicates(&mut nums2));  // 7, nums = [0, 0, 1, 1, 2, 3, 3, _, _]

    let mut nums3 = vec![1];
    println!("{}", remove_duplicates(&mut nums3));  // 1, nums = [1]

    let mut nums4 = vec![1, 2];
    println!("{}", remove_duplicates(&mut nums4));  // 2, nums = [1, 2]
}
