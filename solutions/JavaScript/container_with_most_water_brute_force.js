// Approach: Brute Force
// Try every pair (i, j). Area = Math.min(height[i], height[j]) * (j - i). Track the maximum.
//
// Time Complexity: O(n^2)
// Space Complexity: O(1)

function maxAreaBruteForce(height) {
    const n = height.length;
    let maxWater = 0;
    for (let i = 0; i < n; i++) {
        for (let j = i + 1; j < n; j++) {
            const water = Math.min(height[i], height[j]) * (j - i);
            maxWater = Math.max(maxWater, water);
        }
    }
    return maxWater;
}

console.log(maxAreaBruteForce([1, 8, 6, 2, 5, 4, 8, 3, 7])); // 49
console.log(maxAreaBruteForce([1, 1]));                        // 1
