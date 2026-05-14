fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    if x == 0 {
        return true;
    }
    if x % 10 == 0 {
        return false;
    }
    let mut num_reversed = 0;
    let mut original_x = x;
    while original_x > 0 {
        let last_digit = original_x % 10;
        num_reversed = num_reversed * 10 + last_digit;
        original_x /= 10;
    }
    x == num_reversed
}

fn main() {
    println!("{}", is_palindrome(121)); // true
    println!("{}", is_palindrome(-121)); // false
    println!("{}", is_palindrome(10)); // false
}
