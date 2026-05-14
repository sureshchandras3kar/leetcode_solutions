import java.util.LinkedList;
import java.util.Queue;

public class SymmetricTreeBFS {
    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    /**
     * Check if a binary tree is symmetric using BFS with a queue.
     *
     * Time Complexity: O(n) - visit each node once
     * Space Complexity: O(w) - queue width, where w is max nodes at any level
     */
    public boolean isSymmetric(TreeNode root) {
        if (root == null) {
            return true;
        }

        Queue<TreeNode[]> queue = new LinkedList<>();
        queue.offer(new TreeNode[]{root.left, root.right});

        while (!queue.isEmpty()) {
            TreeNode[] pair = queue.poll();
            TreeNode left = pair[0];
            TreeNode right = pair[1];

            // Both nodes are null - continue (symmetric so far)
            if (left == null && right == null) {
                continue;
            }

            // One node is null or values differ - not symmetric
            if (left == null || right == null) {
                return false;
            }
            if (left.val != right.val) {
                return false;
            }

            // Add pairs for next level: left's left with right's right
            // and left's right with right's left (mirror pattern)
            queue.offer(new TreeNode[]{left.left, right.right});
            queue.offer(new TreeNode[]{left.right, right.left});
        }

        return true;
    }

    // Test cases
    public static void main(String[] args) {
        SymmetricTreeBFS sol = new SymmetricTreeBFS();

        // Example 1: Symmetric tree
        //       1
        //      / \
        //     2   2
        //    / \ / \
        //   3  4 4  3
        TreeNode root1 = new TreeNode(1);
        root1.left = new TreeNode(2);
        root1.right = new TreeNode(2);
        root1.left.left = new TreeNode(3);
        root1.left.right = new TreeNode(4);
        root1.right.left = new TreeNode(4);
        root1.right.right = new TreeNode(3);
        System.out.println(sol.isSymmetric(root1)); // true

        // Example 2: Not symmetric
        //       1
        //      / \
        //     2   2
        //      \   \
        //       3   3
        TreeNode root2 = new TreeNode(1);
        root2.left = new TreeNode(2);
        root2.right = new TreeNode(2);
        root2.left.right = new TreeNode(3);
        root2.right.right = new TreeNode(3);
        System.out.println(sol.isSymmetric(root2)); // false

        // Example 3: Single node
        TreeNode root3 = new TreeNode(1);
        System.out.println(sol.isSymmetric(root3)); // true
    }
}
