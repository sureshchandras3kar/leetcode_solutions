// Approach: Two Pointers
// Use left and right pointers, tracking max_left and max_right seen so far.
// Process whichever side has the smaller max — that side's max is the bottleneck.
// water at current position = max_on_that_side - height[current]
// Update max before adding water, then advance the pointer.
//
// Time Complexity: O(n) — single pass
// Space Complexity: O(1) — four variables only

fn trap(height: &[i32]) -> i32 {
    let mut left = 0usize;
    let mut right = height.len().saturating_sub(1);
    let mut max_left = 0i32;
    let mut max_right = 0i32;
    let mut water = 0i32;
    while left < right {
        if max_left <= max_right {
            max_left = max_left.max(height[left]);
            water += max_left - height[left];
            left += 1;
        } else {
            max_right = max_right.max(height[right]);
            water += max_right - height[right];
            right -= 1;
        }
    }
    water
}

fn main() {
    println!("{}", trap(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])); // 6
    println!("{}", trap(&[4, 2, 0, 3, 2, 5]));                    // 9
}
