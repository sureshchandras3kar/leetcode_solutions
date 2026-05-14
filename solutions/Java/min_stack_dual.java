import java.util.Stack;

public class MinStackDual {
    /**
     * Min Stack using two stacks (dual stack approach).
     *
     * Maintains a main stack for all values and a separate min stack
     * that tracks the minimum value at each level.
     *
     * Time: O(1) per operation
     * Space: O(n)
     */

    private Stack<Integer> mainStack;
    private Stack<Integer> minStack;

    public MinStackDual() {
        mainStack = new Stack<>();
        minStack = new Stack<>();
    }

    public void push(int val) {
        mainStack.push(val);
        // Push to minStack the minimum of val and current min
        if (minStack.isEmpty()) {
            minStack.push(val);
        } else {
            minStack.push(Math.min(val, minStack.peek()));
        }
    }

    public void pop() {
        mainStack.pop();
        minStack.pop();
    }

    public int top() {
        return mainStack.peek();
    }

    public int getMin() {
        return minStack.peek();
    }

    public static void main(String[] args) {
        MinStackDual stack = new MinStackDual();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        System.out.println("Min: " + stack.getMin());  // -3
        stack.pop();
        System.out.println("Top: " + stack.top());     // 0
        System.out.println("Min: " + stack.getMin());  // -2
    }
}
