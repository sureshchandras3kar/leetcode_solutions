/*
Two-Pointer In-Place Approach
Remove duplicates from sorted array in-place and return the length of the new array.

Time Complexity: O(n)
Space Complexity: O(1)
*/
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut k = 1;  // First element is always unique
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            nums[k] = nums[i];
            k += 1;
        }
    }

    k as i32
}

fn main() {
    let mut nums1 = vec![1, 1, 2];
    println!("{}", remove_duplicates(&mut nums1));  // 2, nums = [1, 2, _]

    let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    println!("{}", remove_duplicates(&mut nums2));  // 5, nums = [0, 1, 2, 3, 4, _, _, _, _, _]
}
