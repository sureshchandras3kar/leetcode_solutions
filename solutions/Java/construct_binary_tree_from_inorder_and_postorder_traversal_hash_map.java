import java.util.*;

public class construct_binary_tree_from_inorder_and_postorder_traversal_hash_map {

    static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode(int x) {
            val = x;
        }
    }

    static class Solution {
        private Map<Integer, Integer> inorder_map;

        public TreeNode buildTree(int[] inorder, int[] postorder) {
            /**
             * Construct binary tree from inorder and postorder traversal using HashMap.
             *
             * Key insight:
             * - Postorder: left subtree, right subtree, root (last element is always root)
             * - Inorder: left subtree, root, right subtree
             *
             * Use a HashMap to quickly find the root's position in inorder,
             * then recursively build left and right subtrees.
             *
             * Time Complexity: O(n) where n is the number of nodes
             * Space Complexity: O(n) for HashMap and recursion call stack
             */

            if (inorder.length == 0 || postorder.length == 0) {
                return null;
            }

            // Build HashMap for O(1) inorder lookup
            inorder_map = new HashMap<>();
            for (int i = 0; i < inorder.length; i++) {
                inorder_map.put(inorder[i], i);
            }

            return build(postorder, 0, postorder.length - 1, 0, inorder.length - 1);
        }

        private TreeNode build(int[] postorder, int post_start, int post_end,
                              int in_start, int in_end) {
            /**
             * Recursively build tree from postorder and inorder ranges.
             *
             * Args:
             *     post_start, post_end: Range in postorder array
             *     in_start, in_end: Range in inorder array
             */

            if (post_start > post_end || in_start > in_end) {
                return null;
            }

            // Last element in postorder range is the root
            int root_val = postorder[post_end];
            TreeNode root = new TreeNode(root_val);

            // Find root position in inorder
            int root_idx = inorder_map.get(root_val);

            // Number of nodes in left subtree
            int left_size = root_idx - in_start;

            // Recursively build left subtree
            root.left = build(postorder, post_start, post_start + left_size - 1,
                             in_start, root_idx - 1);

            // Recursively build right subtree
            root.right = build(postorder, post_start + left_size, post_end - 1,
                              root_idx + 1, in_end);

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
