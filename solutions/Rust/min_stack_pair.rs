/**
 * Min Stack using single stack with (value, min) pairs.
 *
 * Each element in the stack is a tuple of (value, min_at_this_level).
 * The min_at_this_level is the minimum value from the bottom up to
 * and including the current element.
 *
 * Time: O(1) per operation
 * Space: O(n)
 */
struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            // First element: value is the minimum
            self.stack.push((val, val));
        } else {
            // Store both value and the minimum up to this point
            let current_min = val.min(self.stack.last().unwrap().1);
            self.stack.push((val, current_min));
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

fn main() {
    let mut stack = MinStack::new();
    stack.push(-2);
    stack.push(0);
    stack.push(-3);
    println!("Min: {}", stack.get_min());  // -3
    stack.pop();
    println!("Top: {}", stack.top());      // 0
    println!("Min: {}", stack.get_min());  // -2
}
