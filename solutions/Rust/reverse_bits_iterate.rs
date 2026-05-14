fn reverse_bits_iterate(mut n: u32) -> u32 {
    let mut result = 0;
    for _ in 0..32 {
        result = (result << 1) | (n & 1);
        n >>= 1;
    }
    result
}

fn main() {
    println!("{}", reverse_bits_iterate(43261596));  // 964176192
}
