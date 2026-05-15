class Solution {
    private int maxSum;

    public int maxPathSum(TreeNode root) {
        maxSum = Integer.MIN_VALUE;
        dfs(root);
        return maxSum;
    }

    private int dfs(TreeNode node) {
        if (node == null) return 0;

        int leftGain = Math.max(0, dfs(node.left));
        int rightGain = Math.max(0, dfs(node.right));

        int maxPathThroughNode = node.val + leftGain + rightGain;
        maxSum = Math.max(maxSum, maxPathThroughNode);

        return node.val + Math.max(leftGain, rightGain);
    }
}

System.out.println(new Solution().maxPathSum(new TreeNode(1, new TreeNode(2), new TreeNode(3))));
