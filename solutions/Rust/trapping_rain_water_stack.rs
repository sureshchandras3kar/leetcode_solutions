// Approach: Monotonic Stack
// Maintain a stack of indices with decreasing heights (a monotonic decreasing stack).
// When height[i] > height[stack.last()], we found a right wall — the pool can be filled.
// Pop the bottom index, compute water between the new stack top (left wall) and i (right wall).
// Think horizontally: each pop fills one horizontal layer of trapped water.
//
// Time Complexity: O(n) — each bar is pushed and popped at most once
// Space Complexity: O(n) — stack stores indices

fn trap(height: &[i32]) -> i32 {
    let mut stack: Vec<usize> = Vec::new();
    let mut water = 0i32;
    for i in 0..height.len() {
        while let Some(&top) = stack.last() {
            if height[i] <= height[top] {
                break;
            }
            stack.pop();
            if let Some(&left) = stack.last() {
                let width = (i - left - 1) as i32;
                let bounded_height = height[left].min(height[i]) - height[top];
                water += bounded_height * width;
            } else {
                break;
            }
        }
        stack.push(i);
    }
    water
}

fn main() {
    println!("{}", trap(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])); // 6
    println!("{}", trap(&[4, 2, 0, 3, 2, 5]));                    // 9
}
