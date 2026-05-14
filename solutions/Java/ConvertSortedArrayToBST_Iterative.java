import java.util.*;

class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;
    TreeNode(int val) {
        this.val = val;
    }
}

class Solution {
    public TreeNode sortedArrayToBST(int[] nums) {
        if (nums.length == 0) return null;

        TreeNode root = new TreeNode(0);
        Queue<Object[]> queue = new LinkedList<>();
        queue.offer(new Object[]{root, 0, nums.length - 1});

        while (!queue.isEmpty()) {
            Object[] item = queue.poll();
            TreeNode node = (TreeNode) item[0];
            int left = (int) item[1];
            int right = (int) item[2];

            int mid = left + (right - left) / 2;
            node.val = nums[mid];

            if (left <= mid - 1) {
                node.left = new TreeNode(0);
                queue.offer(new Object[]{node.left, left, mid - 1});
            }

            if (mid + 1 <= right) {
                node.right = new TreeNode(0);
                queue.offer(new Object[]{node.right, mid + 1, right});
            }
        }

        return root;
    }
}

public class ConvertSortedArrayToBST_Iterative {
    public static void main(String[] args) {
        Solution sol = new Solution();
        int[] nums = {-10, -3, 0, 5, 9};
        TreeNode root = sol.sortedArrayToBST(nums);
        System.out.println(root.val);  // 0
        System.out.println(root.left.val);  // -3
        System.out.println(root.right.val);  // 5
    }
}
