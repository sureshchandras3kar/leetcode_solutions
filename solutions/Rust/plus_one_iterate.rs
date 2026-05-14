fn plus_one_iterate(mut digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 1;
    for i in (0..digits.len()).rev() {
        digits[i] += carry;
        if digits[i] < 10 {
            return digits;
        }
        digits[i] = 0;
    }

    let mut result = vec![1];
    result.extend(digits);
    result
}

fn main() {
    println!("{:?}", plus_one_iterate(vec![1, 2, 3]));  // [1, 2, 4]
    println!("{:?}", plus_one_iterate(vec![9, 9, 9]));  // [1, 0, 0, 0]
}
