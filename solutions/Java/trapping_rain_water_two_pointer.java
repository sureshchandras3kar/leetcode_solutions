// Approach: Two Pointers
// Use left and right pointers, tracking maxLeft and maxRight seen so far.
// Process whichever side has the smaller max — that side's max is the bottleneck.
// water at current position = max_on_that_side - height[current]
// Update max before adding water, then advance the pointer.
//
// Time Complexity: O(n) — single pass
// Space Complexity: O(1) — four variables only

public class Main {
    public static int trap(int[] height) {
        int left = 0, right = height.length - 1;
        int maxLeft = 0, maxRight = 0;
        int water = 0;
        while (left < right) {
            if (maxLeft <= maxRight) {
                maxLeft = Math.max(maxLeft, height[left]);
                water += maxLeft - height[left];
                left++;
            } else {
                maxRight = Math.max(maxRight, height[right]);
                water += maxRight - height[right];
                right--;
            }
        }
        return water;
    }

    public static void main(String[] args) {
        System.out.println(trap(new int[]{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1})); // 6
        System.out.println(trap(new int[]{4, 2, 0, 3, 2, 5}));                    // 9
    }
}
