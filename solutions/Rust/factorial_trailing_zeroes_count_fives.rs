fn factorial_trailing_zeroes_count_fives(n: i32) -> i32 {
    let mut count = 0;
    let mut power_of_5 = 5i64;
    while power_of_5 <= n as i64 {
        count += n as i64 / power_of_5;
        power_of_5 *= 5;
    }
    count as i32
}

fn main() {
    println!("{}", factorial_trailing_zeroes_count_fives(5));  // 1
    println!("{}", factorial_trailing_zeroes_count_fives(25));  // 6
}
