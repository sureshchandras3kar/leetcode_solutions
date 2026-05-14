fn add_binary_iterative(a: String, b: String) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let mut a_chars: Vec<char> = a.chars().collect();
    let mut b_chars: Vec<char> = b.chars().collect();
    let mut i = a_chars.len() as i32 - 1;
    let mut j = b_chars.len() as i32 - 1;

    while i >= 0 || j >= 0 || carry > 0 {
        let digit_a = if i >= 0 { (a_chars[i as usize] as u32 - '0' as u32) as i32 } else { 0 };
        let digit_b = if j >= 0 { (b_chars[j as usize] as u32 - '0' as u32) as i32 } else { 0 };
        let total = digit_a + digit_b + carry;
        result.insert(0, (char::from_digit((total % 2) as u32, 10).unwrap()));
        carry = total / 2;
        i -= 1;
        j -= 1;
    }

    result
}

fn main() {
    println!("{}", add_binary_iterative("11".to_string(), "1".to_string()));  // "100"
}
