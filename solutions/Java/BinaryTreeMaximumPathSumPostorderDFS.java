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
    private long maxSum = Long.MIN_VALUE;

    private long dfs(TreeNode node) {
        if (node == null) return 0;

        // Max gain from left and right subtrees (at least 0 if negative)
        long leftGain = Math.max(0L, dfs(node.left));
        long rightGain = Math.max(0L, dfs(node.right));

        // Max path through this node (may bend at this node)
        long maxPathThroughNode = node.val + leftGain + rightGain;
        maxSum = Math.max(maxSum, maxPathThroughNode);

        // Return max path extending downward from this node
        return node.val + Math.max(leftGain, rightGain);
    }

    /**
     * Find maximum path sum in binary tree using post-order DFS.
     * A path can pass through any node (not just root to leaf).
     * Time: O(n), Space: O(h) for recursion stack
     */
    public int maxPathSum(TreeNode root) {
        dfs(root);
        return (int)maxSum;
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
