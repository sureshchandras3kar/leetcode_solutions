import java.util.*;

public class construct_binary_tree_from_inorder_and_postorder_traversal_index {

    static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode(int x) {
            val = x;
        }
    }

    static class Solution {
        private int post_idx;

        public TreeNode buildTree(int[] inorder, int[] postorder) {
            /**
             * Construct binary tree from inorder and postorder traversal using index tracking.
             *
             * Key insight:
             * - Postorder: left subtree, right subtree, root (last element is always root)
             * - Inorder: left subtree, root, right subtree
             * - Use a pointer to track the current root in postorder (traverse from end to start)
             * - Find root in inorder to split into left and right subtrees
             *
             * Time Complexity: O(n²) in worst case due to linear search for root in inorder
             * Space Complexity: O(h) for recursion call stack where h is height
             */

            if (inorder.length == 0 || postorder.length == 0) {
                return null;
            }

            post_idx = postorder.length - 1;
            return build(postorder, inorder, 0, inorder.length - 1);
        }

        private TreeNode build(int[] postorder, int[] inorder, int in_start, int in_end) {
            /**
             * Recursively build tree by processing postorder from right to left.
             *
             * Args:
             *     in_start, in_end: Range in inorder array
             */

            if (in_start > in_end || post_idx < 0) {
                return null;
            }

            // Current postorder element (processing from end to start)
            int root_val = postorder[post_idx];
            post_idx--;

            TreeNode root = new TreeNode(root_val);

            // Find root position in inorder
            int root_idx = -1;
            for (int i = in_start; i <= in_end; i++) {
                if (inorder[i] == root_val) {
                    root_idx = i;
                    break;
                }
            }

            // Build right subtree first (postorder: left, right, root)
            // Since we traverse postorder backwards, right comes before left
            root.right = build(postorder, inorder, root_idx + 1, in_end);

            // Build left subtree
            root.left = build(postorder, inorder, in_start, root_idx - 1);

            return root;
        }
    }

    // Helper function for inorder traversal
    static List<Integer> inorder_traversal(TreeNode node) {
        List<Integer> result = new ArrayList<>();
        inorder_helper(node, result);
        return result;
    }

    static void inorder_helper(TreeNode node, List<Integer> result) {
        if (node == null) return;
        inorder_helper(node.left, result);
        result.add(node.val);
        inorder_helper(node.right, result);
    }

    // Helper function for postorder traversal
    static List<Integer> postorder_traversal(TreeNode node) {
        List<Integer> result = new ArrayList<>();
        postorder_helper(node, result);
        return result;
    }

    static void postorder_helper(TreeNode node, List<Integer> result) {
        if (node == null) return;
        postorder_helper(node.left, result);
        postorder_helper(node.right, result);
        result.add(node.val);
    }

    public static void main(String[] args) {
        Solution sol = new Solution();

        // Example 1: [3,9,20,null,null,15,7]
        //     3
        //    / \
        //   9  20
        //      / \
        //     15  7
        int[] inorder1 = {9, 3, 15, 20, 7};
        int[] postorder1 = {9, 15, 7, 20, 3};
        TreeNode root1 = sol.buildTree(inorder1, postorder1);

        System.out.print("Inorder: ");
        System.out.println(inorder_traversal(root1));  // Expected: [9, 3, 15, 20, 7]

        System.out.print("Postorder: ");
        System.out.println(postorder_traversal(root1));  // Expected: [9, 15, 7, 20, 3]

        // Example 2: Single node
        int[] inorder2 = {1};
        int[] postorder2 = {1};
        TreeNode root2 = sol.buildTree(inorder2, postorder2);
        System.out.println("Single node inorder: " + inorder_traversal(root2));

        // Example 3: Left skewed tree
        int[] inorder3 = {3, 2, 1};
        int[] postorder3 = {3, 2, 1};
        TreeNode root3 = sol.buildTree(inorder3, postorder3);
        System.out.println("Left skewed inorder: " + inorder_traversal(root3));
    }
}
