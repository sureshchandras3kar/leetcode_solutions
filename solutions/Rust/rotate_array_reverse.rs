fn rotate_array_reverse(nums: &mut Vec<i32>, mut k: i32) {
    /**
     * Rotate array in-place using reverse trick.
     *
     * Time: O(n) | Space: O(1)
     *
     * Approach: Reverse the entire array, then reverse first k elements,
     * then reverse remaining n-k elements. This achieves rotation without
     * extra space.
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

    fn reverse(nums: &mut Vec<i32>, mut start: usize, mut end: usize) {
        while start < end {
            nums.swap(start, end);
            start += 1;
            end -= 1;
        }
    }

    // Reverse entire array: [1,2,3,4,5] -> [5,4,3,2,1]
    reverse(nums, 0, n - 1);
    // Reverse first k: [5,4,3,2,1] -> [3,4,5,2,1]
    reverse(nums, 0, k - 1);
    // Reverse rest: [3,4,5,2,1] -> [3,4,5,1,2]
    reverse(nums, k, n - 1);
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    rotate_array_reverse(&mut nums, 2);
    println!("{:?}", nums);  // [4, 5, 1, 2, 3]
}
