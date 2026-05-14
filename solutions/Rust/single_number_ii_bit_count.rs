fn single_number_ii_bit_count(nums: Vec<i32>) -> i32 {
    let mut bit_counts = vec![0; 32];
    for num in nums {
        for i in 0..32 {
            if (num & (1 << i)) != 0 {
                bit_counts[i] += 1;
            }
        }
    }

    let mut result = 0;
    for i in 0..32 {
        if bit_counts[i] % 3 != 0 {
            result |= 1 << i;
        }
    }

    if result >= (1 << 31) {
        result -= 1 << 32;
    }
    result
}

fn main() {
    println!("{}", single_number_ii_bit_count(vec![2, 2, 3, 2]));  // 3
}
