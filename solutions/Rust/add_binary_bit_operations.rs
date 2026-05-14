pub fn add_binary_bit_operations(a: String, b: String) -> String {
    /*
    Bit operations approach - convert to u64, add, convert back to binary.
    Time: O(max(a.len(), b.len()))
    Space: O(max(a.len(), b.len())) for result
    */
    let num_a = u64::from_str_radix(&a, 2).unwrap();
    let num_b = u64::from_str_radix(&b, 2).unwrap();
    let total = num_a + num_b;
    format!("{:b}", total)
}

fn main() {
    println!("{}", add_binary_bit_operations("11".to_string(), "1".to_string()));      // "100"
    println!("{}", add_binary_bit_operations("1010".to_string(), "1011".to_string())); // "10101"
    println!("{}", add_binary_bit_operations("0".to_string(), "0".to_string()));       // "0"
}
