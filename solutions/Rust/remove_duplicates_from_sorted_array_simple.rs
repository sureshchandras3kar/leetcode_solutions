/*
Simple Pass Approach
Remove duplicates by iterating and comparing consecutive elements.

Time Complexity: O(n)
Space Complexity: O(1)
*/
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut write_idx = 0;
    for read_idx in 1..nums.len() {
        if nums[read_idx] != nums[write_idx] {
            write_idx += 1;
            nums[write_idx] = nums[read_idx];
        }
    }

    (write_idx + 1) as i32
}

fn main() {
    let mut nums1 = vec![1, 1, 2];
    println!("{}", remove_duplicates(&mut nums1));  // 2, nums = [1, 2, _]

    let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    println!("{}", remove_duplicates(&mut nums2));  // 5, nums = [0, 1, 2, 3, 4, _, _, _, _, _]
}
