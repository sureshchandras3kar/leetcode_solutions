fn powx_n_fast_exponentiation(mut x: f64, mut n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }

    let mut N = n as i64;
    if N < 0 {
        x = 1.0 / x;
        N = -N;
    }

    let mut result = 1.0;
    while N > 0 {
        if N % 2 == 1 {
            result *= x;
        }
        x *= x;
        N /= 2;
    }

    result
}

fn main() {
    println!("{}", powx_n_fast_exponentiation(2.0, 10));  // 1024.0
    println!("{}", powx_n_fast_exponentiation(2.1, 3));  // 9.261
}
