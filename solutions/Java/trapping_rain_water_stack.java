// Approach: Monotonic Stack
// Maintain a stack of indices with decreasing heights (a monotonic decreasing stack).
// When height[i] > height[stack.peek()], we found a right wall — the pool can be filled.
// Pop the bottom index, compute water between the new stack top (left wall) and i (right wall).
// Think horizontally: each pop fills one horizontal layer of trapped water.
//
// Time Complexity: O(n) — each bar is pushed and popped at most once
// Space Complexity: O(n) — stack stores indices

import java.util.Deque;
import java.util.ArrayDeque;

public class Main {
    public static int trap(int[] height) {
        Deque<Integer> stack = new ArrayDeque<>();
        int water = 0;
        for (int i = 0; i < height.length; i++) {
            while (!stack.isEmpty() && height[i] > height[stack.peek()]) {
                int bottom = stack.pop();
                if (stack.isEmpty()) break;
                int left = stack.peek();
                int width = i - left - 1;
                int boundedHeight = Math.min(height[left], height[i]) - height[bottom];
                water += boundedHeight * width;
            }
            stack.push(i);
        }
        return water;
    }

    public static void main(String[] args) {
        System.out.println(trap(new int[]{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1})); // 6
        System.out.println(trap(new int[]{4, 2, 0, 3, 2, 5}));                    // 9
    }
}
