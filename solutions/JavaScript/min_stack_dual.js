/**
 * Min Stack using two stacks (dual stack approach).
 *
 * Maintains a main stack for all values and a separate min stack
 * that tracks the minimum value at each level.
 *
 * Time: O(1) per operation
 * Space: O(n)
 */
class MinStack {
  constructor() {
    this.mainStack = [];
    this.minStack = [];
  }

  push(val) {
    this.mainStack.push(val);
    // Push to minStack the minimum of val and current min
    if (this.minStack.length === 0) {
      this.minStack.push(val);
    } else {
      this.minStack.push(Math.min(val, this.minStack[this.minStack.length - 1]));
    }
  }

  pop() {
    this.mainStack.pop();
    this.minStack.pop();
  }

  top() {
    return this.mainStack[this.mainStack.length - 1];
  }

  getMin() {
    return this.minStack[this.minStack.length - 1];
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
