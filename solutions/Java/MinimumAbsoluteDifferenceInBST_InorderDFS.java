public class MinimumAbsoluteDifferenceInBST_InorderDFS {
    static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    int minDiff = Integer.MAX_VALUE;
    TreeNode prev = null;

    private void inorder(TreeNode node) {
        if (node == null) return;

        // Traverse left subtree
        inorder(node.left);

        // Process current node
        if (prev != null) {
            minDiff = Math.min(minDiff, node.val - prev.val);
        }
        prev = node;

        // Traverse right subtree
        inorder(node.right);
    }

    public int getMinimumDifference(TreeNode root) {
        inorder(root);
        return minDiff;
    }

    public static void main(String[] args) {
        TreeNode root = new TreeNode(4);
        root.left = new TreeNode(2);
        root.right = new TreeNode(6);
        root.left.left = new TreeNode(1);
        root.left.right = new TreeNode(3);

        MinimumAbsoluteDifferenceInBST_InorderDFS sol = new MinimumAbsoluteDifferenceInBST_InorderDFS();
        System.out.println(sol.getMinimumDifference(root));  // 1
    }
}
