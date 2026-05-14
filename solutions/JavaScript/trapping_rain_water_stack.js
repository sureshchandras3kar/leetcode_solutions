// Approach: Monotonic Stack
// Maintain a stack of indices with decreasing heights (a monotonic decreasing stack).
// When height[i] > height[stack top], we found a right wall — the pool can be filled.
// Pop the bottom index, compute water between the new stack top (left wall) and i (right wall).
// Think horizontally: each pop fills one horizontal layer of trapped water.
//
// Time Complexity: O(n) — each bar is pushed and popped at most once
// Space Complexity: O(n) — stack stores indices

function trap(height) {
    const stack = [];
    let water = 0;
    for (let i = 0; i < height.length; i++) {
        while (stack.length > 0 && height[i] > height[stack[stack.length - 1]]) {
            const bottom = stack.pop();
            if (stack.length === 0) break;
            const left = stack[stack.length - 1];
            const width = i - left - 1;
            const boundedHeight = Math.min(height[left], height[i]) - height[bottom];
            water += boundedHeight * width;
        }
        stack.push(i);
    }
    return water;
}

console.log(trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])); // 6
console.log(trap([4, 2, 0, 3, 2, 5]));                    // 9
