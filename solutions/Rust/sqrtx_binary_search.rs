fn sqrtx_binary_search(x: i32) -> i32 {
    if x < 2 {
        return x;
    }

    let mut left = 2i64;
    let mut right = (x / 2) as i64;

    while left <= right {
        let mid = (left + right) / 2;
        if mid * mid == x as i64 {
            return mid as i32;
        } else if mid * mid < x as i64 {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    right as i32
}

fn main() {
    println!("{}", sqrtx_binary_search(4));  // 2
    println!("{}", sqrtx_binary_search(8));  // 2
}
