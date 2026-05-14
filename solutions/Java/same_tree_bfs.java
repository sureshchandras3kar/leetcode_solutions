import java.util.*;

public class same_tree_bfs {
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
     * Check if two binary trees are the same using BFS (level-order traversal).
     *
     * Time Complexity: O(min(m, n)) where m and n are the number of nodes
     * Space Complexity: O(min(w1, w2)) where w1 and w2 are the widths
     */
    public static boolean isSameTree(TreeNode p, TreeNode q) {
        Queue<TreeNode[]> queue = new LinkedList<>();
        queue.add(new TreeNode[]{p, q});

        while (!queue.isEmpty()) {
            TreeNode[] nodes = queue.poll();
            TreeNode node1 = nodes[0];
            TreeNode node2 = nodes[1];

            // Both are null (equal)
            if (node1 == null && node2 == null) {
                continue;
            }

            // One is null or values differ (not equal)
            if (node1 == null || node2 == null || node1.val != node2.val) {
                return false;
            }

            // Add children to queue for comparison
            queue.add(new TreeNode[]{node1.left, node2.left});
            queue.add(new TreeNode[]{node1.right, node2.right});
        }

        return true;
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
