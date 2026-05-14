/**
 * Min Stack using two stacks (dual stack approach).
 *
 * Maintains a main stack for all values and a separate min stack
 * that tracks the minimum value at each level.
 *
 * Time: O(1) per operation
 * Space: O(n)
 */
struct MinStack {
    main_stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            main_stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.main_stack.push(val);
        // Push to min_stack the minimum of val and current min
        if self.min_stack.is_empty() {
            self.min_stack.push(val);
        } else {
            let current_min = (*self.min_stack.last().unwrap()).min(val);
            self.min_stack.push(current_min);
        }
    }

    fn pop(&mut self) {
        self.main_stack.pop();
        self.min_stack.pop();
    }

    fn top(&self) -> i32 {
        *self.main_stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
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
