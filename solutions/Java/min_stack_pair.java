import java.util.Stack;

public class MinStackPair {
    /**
     * Min Stack using single stack with (value, min) pairs.
     *
     * Each element in the stack is a pair of (value, min_at_this_level).
     * The min_at_this_level is the minimum value from the bottom up to
     * and including the current element.
     *
     * Time: O(1) per operation
     * Space: O(n)
     */

    private Stack<int[]> stack;

    public MinStackPair() {
        stack = new Stack<>();
    }

    public void push(int val) {
        if (stack.isEmpty()) {
            // First element: value is the minimum
            stack.push(new int[]{val, val});
        } else {
            // Store both value and the minimum up to this point
            int currentMin = Math.min(val, stack.peek()[1]);
            stack.push(new int[]{val, currentMin});
        }
    }

    public void pop() {
        stack.pop();
    }

    public int top() {
        return stack.peek()[0];
    }

    public int getMin() {
        return stack.peek()[1];
    }

    public static void main(String[] args) {
        MinStackPair stack = new MinStackPair();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        System.out.println("Min: " + stack.getMin());  // -3
        stack.pop();
        System.out.println("Top: " + stack.top());     // 0
        System.out.println("Min: " + stack.getMin());  // -2
    }
}
