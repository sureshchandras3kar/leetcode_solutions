/**
 * Divide and Conquer approach: Split the array, find max subarrays in each half,
 * and at the crossing point, then return the maximum.
 * Time: O(n log n), Space: O(log n) for recursion stack
 */
pub fn maximum_subarray_divide_conquer(nums: Vec<i32>) -> i32 {
    fn max_crossing_subarray(nums: &[i32], left: i32, mid: i32, right: i32) -> i32 {
        // Sum from mid to left
        let mut left_sum = i32::MIN;
        let mut sum = 0;
        let mut i = mid;
        loop {
            sum += nums[i as usize];
            left_sum = left_sum.max(sum);
            if i == left {
                break;
            }
            i -= 1;
        }

        // Sum from mid+1 to right
        let mut right_sum = i32::MIN;
        sum = 0;
        for j in (mid + 1)..=right {
            sum += nums[j as usize];
            right_sum = right_sum.max(sum);
        }

        left_sum + right_sum
    }

    fn helper(nums: &[i32], left: i32, right: i32) -> i32 {
        if left == right {
            return nums[left as usize];
        }

        let mid = left + (right - left) / 2;

        // Maximum in left half, right half, and crossing the middle
        let left_max = helper(nums, left, mid);
        let right_max = helper(nums, mid + 1, right);
        let cross_max = max_crossing_subarray(nums, left, mid, right);

        left_max.max(right_max).max(cross_max)
    }

    helper(&nums, 0, (nums.len() - 1) as i32)
}

fn main() {
    println!("{}", maximum_subarray_divide_conquer(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));  // 6
    println!("{}", maximum_subarray_divide_conquer(vec![5]));  // 5
    println!("{}", maximum_subarray_divide_conquer(vec![-5, -2, -3]));  // -2
}
