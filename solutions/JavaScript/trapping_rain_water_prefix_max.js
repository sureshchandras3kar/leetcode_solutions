// Approach: Prefix Max Arrays
// Build leftMax[i] = max height from height[0] to height[i].
// Build rightMax[i] = max height from height[i] to height[n-1].
// Water at i = Math.max(0, Math.min(leftMax[i], rightMax[i]) - height[i]).
// The minimum of both maxes is the effective wall height that bounds the water.
//
// Time Complexity: O(n) — three linear passes
// Space Complexity: O(n) — two extra arrays of size n

function trap(height) {
    const n = height.length;
    if (n === 0) return 0;

    const leftMax = new Array(n);
    const rightMax = new Array(n);

    leftMax[0] = height[0];
    for (let i = 1; i < n; i++)
        leftMax[i] = Math.max(leftMax[i - 1], height[i]);

    rightMax[n - 1] = height[n - 1];
    for (let i = n - 2; i >= 0; i--)
        rightMax[i] = Math.max(rightMax[i + 1], height[i]);

    let water = 0;
    for (let i = 0; i < n; i++)
        water += Math.max(0, Math.min(leftMax[i], rightMax[i]) - height[i]);

    return water;
}

console.log(trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])); // 6
console.log(trap([4, 2, 0, 3, 2, 5]));                    // 9
