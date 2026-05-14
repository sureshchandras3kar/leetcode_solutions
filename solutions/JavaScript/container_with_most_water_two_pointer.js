// Approach: Two Pointers
// Place one pointer at the start and one at the end.
// Area = Math.min(height[left], height[right]) * (right - left).
// Move the pointer with the shorter height inward — the shorter wall is the bottleneck.
//
// Time Complexity: O(n)
// Space Complexity: O(1)

function maxArea(height) {
    let left = 0;
    let right = height.length - 1;
    let maxWater = 0;
    while (left < right) {
        const water = Math.min(height[left], height[right]) * (right - left);
        maxWater = Math.max(maxWater, water);
        if (height[left] <= height[right]) {
            left++;
        } else {
            right--;
        }
    }
    return maxWater;
}

console.log(maxArea([1, 8, 6, 2, 5, 4, 8, 3, 7])); // 49
console.log(maxArea([1, 1]));                        // 1
