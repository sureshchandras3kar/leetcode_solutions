class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;

    TreeNode() {}
    TreeNode(int val) {
        this.val = val;
    }
}

class Solution {
    /**
     * Flatten binary tree to linked list using pre-order DFS.
     * Recursively flattens left and right subtrees, then rewires pointers.
     * Time: O(n), Space: O(h) for recursion stack
     */
    public void flatten(TreeNode root) {
        if (root == null) return;

        flatten(root.left);
        flatten(root.right);

        if (root.left != null) {
            // Find rightmost node in flattened left subtree
            TreeNode rightmost = root.left;
            while (rightmost.right != null) {
                rightmost = rightmost.right;
            }

            // Attach right subtree to rightmost node
            rightmost.right = root.right;
            // Move flattened left subtree to right
            root.right = root.left;
            root.left = null;
        }
    }
}

class Main {
    public static void main(String[] args) {
        // Example 1: [1,2,5,3,4,null,6]
        TreeNode root = new TreeNode(1);
        root.left = new TreeNode(2);
        root.left.left = new TreeNode(3);
        root.left.right = new TreeNode(4);
        root.right = new TreeNode(5);
        root.right.right = new TreeNode(6);

        Solution sol = new Solution();
        sol.flatten(root);

        System.out.println("Example 1: Tree flattened to linked list using pre-order DFS");
    }
}
