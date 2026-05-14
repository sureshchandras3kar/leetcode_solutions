import java.util.LinkedList;
import java.util.Queue;

public class invert_binary_tree_bfs {

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
         * BFS Iterative approach to invert a binary tree.
         * Uses a queue to visit nodes level by level, swapping children.
         *
         * Time Complexity: O(n) - visit each node once
         * Space Complexity: O(w) - w = max width of tree (nodes at widest level)
         */
        public TreeNode invertTree(TreeNode root) {
            if (root == null) {
                return root;
            }

            Queue<TreeNode> queue = new LinkedList<>();
            queue.add(root);

            while (!queue.isEmpty()) {
                TreeNode node = queue.poll();

                // Swap left and right children
                TreeNode temp = node.left;
                node.left = node.right;
                node.right = temp;

                // Add children to queue for processing
                if (node.left != null) {
                    queue.add(node.left);
                }
                if (node.right != null) {
                    queue.add(node.right);
                }
            }

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
