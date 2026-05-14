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
     * Find maximum path sum using DFS with mutable max tracking.
     * Maintains maxSum as instance variable updated during traversal.
     * Time: O(n), Space: O(h) for recursion stack
     */
    public int maxPathSum(TreeNode root) {
        long[] maxSum = {Long.MIN_VALUE};
        dfs(root, maxSum);
        return (int)maxSum[0];
    }

    private long dfs(TreeNode node, long[] maxSum) {
        if (node == null) return 0;

        // Max single-path sum extending from this node
        long leftSum = Math.max(0L, dfs(node.left, maxSum));
        long rightSum = Math.max(0L, dfs(node.right, maxSum));

        // Path bending at this node
        long pathSum = node.val + leftSum + rightSum;
        maxSum[0] = Math.max(maxSum[0], pathSum);

        // Return best single path extending downward
        return node.val + Math.max(leftSum, rightSum);
    }
}

class Main {
    public static void main(String[] args) {
        // Example 1: [1,2,3]
        TreeNode root = new TreeNode(1);
        root.left = new TreeNode(2);
        root.right = new TreeNode(3);

        Solution sol = new Solution();
        System.out.println("Example 1 max path: " + sol.maxPathSum(root));  // 6 (2 -> 1 -> 3)

        // Example 2: [-10,9,20,null,null,15,7]
        TreeNode root2 = new TreeNode(-10);
        root2.left = new TreeNode(9);
        root2.right = new TreeNode(20);
        root2.right.left = new TreeNode(15);
        root2.right.right = new TreeNode(7);

        Solution sol2 = new Solution();
        System.out.println("Example 2 max path: " + sol2.maxPathSum(root2));  // 42 (15 -> 20 -> 7)
    }
}
