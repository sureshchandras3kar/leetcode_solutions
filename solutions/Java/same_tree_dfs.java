import java.util.*;

public class same_tree_dfs {
    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode() {}

        TreeNode(int val) {
            this.val = val;
        }

        TreeNode(int val, TreeNode left, TreeNode right) {
            this.val = val;
            this.left = left;
            this.right = right;
        }
    }

    /**
     * Check if two binary trees are the same using DFS (recursive).
     *
     * Time Complexity: O(min(m, n)) where m and n are the number of nodes
     * Space Complexity: O(min(h1, h2)) where h1 and h2 are the heights (call
     * stack)
     */
    public static boolean isSameTree(TreeNode p, TreeNode q) {
        // Both nodes are null (base case: equal)
        if (p == null && q == null) {
            return true;
        }

        // One is null, the other isn't (not equal)
        if (p == null || q == null) {
            return false;
        }

        // Values differ (not equal)
        if (p.val != q.val) {
            return false;
        }

        // Recursively check left and right subtrees
        return isSameTree(p.left, q.left) && isSameTree(p.right, q.right);
    }

    public static void main(String[] args) {
        // Example 1: Same trees
        //     1           1
        //    / \         / \
        //   2   3       2   3
        TreeNode p1 = new TreeNode(1);
        p1.left = new TreeNode(2);
        p1.right = new TreeNode(3);

        TreeNode q1 = new TreeNode(1);
        q1.left = new TreeNode(2);
        q1.right = new TreeNode(3);

        System.out.println(isSameTree(p1, q1)); // Expected: true

        // Example 2: Different structure
        //     1           1
        //    /             \
        //   2               2
        TreeNode p2 = new TreeNode(1);
        p2.left = new TreeNode(2);

        TreeNode q2 = new TreeNode(1);
        q2.right = new TreeNode(2);

        System.out.println(isSameTree(p2, q2)); // Expected: false

        // Example 3: Different values
        //     1           1
        //    / \         / \
        //   2   1       1   2
        TreeNode p3 = new TreeNode(1);
        p3.left = new TreeNode(2);
        p3.right = new TreeNode(1);

        TreeNode q3 = new TreeNode(1);
        q3.left = new TreeNode(1);
        q3.right = new TreeNode(2);

        System.out.println(isSameTree(p3, q3)); // Expected: false

        // Example 4: Both empty
        System.out.println(isSameTree(null, null)); // Expected: true
    }
}
