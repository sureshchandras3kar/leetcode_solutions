impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut tails = Vec::new();

        for num in nums {
            match tails.binary_search(&num) {
                Ok(pos) => tails[pos] = num,
                Err(pos) => {
                    if pos == tails.len() {
                        tails.push(num);
                    } else {
                        tails[pos] = num;
                    }
                }
            }
        }

        tails.len() as i32
    }
}

pub struct Solution;
