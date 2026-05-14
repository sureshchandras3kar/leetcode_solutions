fn rotate_array_extra_space(nums: &mut Vec<i32>, mut k: i32) {
    /**
     * Rotate array using extra space.
     *
     * Time: O(n) | Space: O(n)
     *
     * Approach: Create a new array where element at index i goes to index
     * (i + k) % n. Copy back to original array.
     */
    if nums.is_empty() || k == 0 {
        return;
    }

    let n = nums.len();
    k = k % (n as i32);  // Handle k > n
    if k == 0 {
        return;
    }

    let k = k as usize;

    // Create rotated result
    let mut rotated = vec![0; n];
    for i in 0..n {
        rotated[(i + k) % n] = nums[i];
    }

    // Copy back to original array
    for i in 0..n {
        nums[i] = rotated[i];
    }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    rotate_array_extra_space(&mut nums, 2);
    println!("{:?}", nums);  // [4, 5, 1, 2, 3]
}
