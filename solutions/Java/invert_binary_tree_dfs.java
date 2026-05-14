import java.util.LinkedList;
import java.util.Queue;

public class invert_binary_tree_dfs {

    static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int val) {
            this.val = val;
        }
    }

    static class Solution {
        /**
         * DFS Recursive approach to invert a binary tree.
         * Recursively swap left and right children for each node.
         *
         * Time Complexity: O(n) - visit each node once
         * Space Complexity: O(h) - recursion stack depth (h = height)
         */
        public TreeNode invertTree(TreeNode root) {
            if (root == null) {
                return null;
            }

            // Swap left and right children
            TreeNode temp = root.left;
            root.left = root.right;
            root.right = temp;

            // Recursively invert left and right subtrees
            invertTree(root.left);
            invertTree(root.right);

            return root;
        }
    }

    public static void main(String[] args) {
        // Create tree:     1
        //                /   \
        //               2     3
        TreeNode root = new TreeNode(1);
        root.left = new TreeNode(2);
        root.right = new TreeNode(3);

        Solution sol = new Solution();
        sol.invertTree(root);

        // Expected:        1
        //                /   \
        //               3     2
        System.out.println("Root: " + root.val);  // 1
        System.out.println("Left: " + root.left.val + ", Right: " + root.right.val);  // 3, 2
    }
}
