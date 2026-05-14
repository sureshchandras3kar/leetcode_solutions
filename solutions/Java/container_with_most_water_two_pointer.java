// Approach: Two Pointers
// Place one pointer at the start and one at the end.
// Area = min(height[left], height[right]) * (right - left).
// Move the pointer with the shorter height inward — the shorter wall is the bottleneck.
//
// Time Complexity: O(n)
// Space Complexity: O(1)

public class Main {
    public static int maxArea(int[] height) {
        int left = 0;
        int right = height.length - 1;
        int maxWater = 0;
        while (left < right) {
            int water = Math.min(height[left], height[right]) * (right - left);
            maxWater = Math.max(maxWater, water);
            if (height[left] <= height[right]) {
                left++;
            } else {
                right--;
            }
        }
        return maxWater;
    }

    public static void main(String[] args) {
        System.out.println(maxArea(new int[]{1, 8, 6, 2, 5, 4, 8, 3, 7})); // 49
        System.out.println(maxArea(new int[]{1, 1}));                       // 1
    }
}
