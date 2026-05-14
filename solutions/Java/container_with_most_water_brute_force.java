// Approach: Brute Force
// Try every pair (i, j). Area = min(height[i], height[j]) * (j - i). Track the maximum.
//
// Time Complexity: O(n^2)
// Space Complexity: O(1)

public class Main {
    public static int maxAreaBruteForce(int[] height) {
        int n = height.length;
        int maxWater = 0;
        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                int water = Math.min(height[i], height[j]) * (j - i);
                maxWater = Math.max(maxWater, water);
            }
        }
        return maxWater;
    }

    public static void main(String[] args) {
        System.out.println(maxAreaBruteForce(new int[]{1, 8, 6, 2, 5, 4, 8, 3, 7})); // 49
        System.out.println(maxAreaBruteForce(new int[]{1, 1}));                       // 1
    }
}
