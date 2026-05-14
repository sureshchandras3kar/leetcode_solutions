/**
 * Min Stack using single stack with (value, min) pairs.
 *
 * Each element in the stack is an object of {value, min}.
 * The min is the minimum value from the bottom up to
 * and including the current element.
 *
 * Time: O(1) per operation
 * Space: O(n)
 */
class MinStack {
  constructor() {
    this.stack = [];
  }

  push(val) {
    if (this.stack.length === 0) {
      // First element: value is the minimum
      this.stack.push({ value: val, min: val });
    } else {
      // Store both value and the minimum up to this point
      const currentMin = Math.min(val, this.stack[this.stack.length - 1].min);
      this.stack.push({ value: val, min: currentMin });
    }
  }

  pop() {
    this.stack.pop();
  }

  top() {
    return this.stack[this.stack.length - 1].value;
  }

  getMin() {
    return this.stack[this.stack.length - 1].min;
  }
}

// Example usage
const stack = new MinStack();
stack.push(-2);
stack.push(0);
stack.push(-3);
console.log(`Min: ${stack.getMin()}`);  // -3
stack.pop();
console.log(`Top: ${stack.top()}`);     // 0
console.log(`Min: ${stack.getMin()}`);  // -2
