fn single_number_xor(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for num in nums {
        result ^= num;
    }
    result
}

fn main() {
    println!("{}", single_number_xor(vec![2, 2, 1]));  // 1
    println!("{}", single_number_xor(vec![4, 1, 2, 1, 2]));  // 4
}
