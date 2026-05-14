/**
 * Find length of longest increasing subsequence using binary search O(n log n).
 *
 * Time Complexity: O(n log n)
 * Space Complexity: O(n)
 */
pub fn lis_binary_search(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut tails = Vec::new();

    for num in nums {
        let pos = tails.binary_search(&num).unwrap_or_else(|e| e);

        if pos == tails.len() {
            tails.push(num);
        } else {
            tails[pos] = num;
        }
    }

    tails.len() as i32
}

fn main() {
    println!("{}", lis_binary_search(vec![10, 9, 2, 5, 3, 7, 101, 18]));  // 4
}
