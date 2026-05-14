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
    private TreeNode prev = null;

    private void dfs(TreeNode node) {
        if (node == null) return;

        // Post-order: right, left, then process node
        dfs(node.right);
        dfs(node.left);

        node.right = prev;
        node.left = null;
        prev = node;
    }

    /**
     * Flatten binary tree to linked list using post-order DFS.
     * Uses previous pointer to track last visited node in reverse in-order.
     * Time: O(n), Space: O(h) for recursion stack
     */
    public void flatten(TreeNode root) {
        dfs(root);
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

        System.out.println("Example 1: Tree flattened to linked list using post-order DFS");
    }
}
