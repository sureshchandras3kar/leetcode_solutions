/*
Simple Two Pointers Approach
Allow at most 2 occurrences by checking if current element differs from element 2 positions back.

Time Complexity: O(n)
Space Complexity: O(1)
*/
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut k = 0;
    for i in 0..nums.len() {
        // Always write first 2 elements, or if current differs from element 2 positions back
        if k < 2 || nums[i] != nums[k - 2] {
            nums[k] = nums[i];
            k += 1;
        }
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
