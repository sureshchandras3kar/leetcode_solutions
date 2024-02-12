fn is_palindrome(x: i32) -> bool {
    let str_x = x.to_string();
    str_x == str_x.chars().rev().collect::<String>()
}

fn main() {
    println!("{}", is_palindrome(121)); // true
    println!("{}", is_palindrome(-121)); // false
    println!("{}", is_palindrome(10)); // false
}
