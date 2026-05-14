fn number_of_1_bits_loop(mut n: u32) -> i32 {
    let mut count = 0;
    while n > 0 {
        count += (n & 1) as i32;
        n >>= 1;
    }
    count
}

fn main() {
    println!("{}", number_of_1_bits_loop(11));  // 3
    println!("{}", number_of_1_bits_loop(128));  // 1
}
