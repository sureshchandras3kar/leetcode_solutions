import java.util.*;

class TreeNode {
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

public class maximum_depth_of_binary_tree_bfs {
    /*
     * Find the maximum depth of a binary tree using BFS (level-order traversal).
     *
     * Time Complexity: O(n) where n is the number of nodes
     * Space Complexity: O(w) where w is the maximum width of the tree
     */
    public static int maxDepth(TreeNode root) {
        if (root == null) {
            return 0;
        }

        Queue<TreeNode> queue = new LinkedList<>();
        queue.offer(root);
        int depth = 0;

        while (!queue.isEmpty()) {
            depth++;
            // Process all nodes at the current level
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                TreeNode node = queue.poll();
                if (node.left != null) {
                    queue.offer(node.left);
                }
                if (node.right != null) {
                    queue.offer(node.right);
                }
            }
        }

        return depth;
    }

    // Test cases
    public static void main(String[] args) {
        // Example 1: [3,9,20,null,null,15,7]
        //       3
        //      / \
        //     9  20
        //       /  \
        //      15   7
        TreeNode root1 = new TreeNode(3);
        root1.left = new TreeNode(9);
        root1.right = new TreeNode(20);
        root1.right.left = new TreeNode(15);
        root1.right.right = new TreeNode(7);

        System.out.println(maxDepth(root1)); // Expected: 3

        // Example 2: [1,null,2]
        //     1
        //      \
        //       2
        TreeNode root2 = new TreeNode(1);
        root2.right = new TreeNode(2);

        System.out.println(maxDepth(root2)); // Expected: 2

        // Example 3: Empty tree
        TreeNode root3 = null;
        System.out.println(maxDepth(root3)); // Expected: 0
    }
}
