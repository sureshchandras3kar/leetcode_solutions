public class Solution {
    boolean dfs(TreeNode n, long minVal, long maxVal) {
        if (n == null) return true;
        if (n.val <= minVal || n.val >= maxVal) return false;
        return dfs(n.left, minVal, n.val) && dfs(n.right, n.val, maxVal);
    }
    public boolean isValidBST(TreeNode root) {
        return dfs(root, Long.MIN_VALUE, Long.MAX_VALUE);
    }
    public static class TreeNode { int val; TreeNode left, right; TreeNode(int x) { val = x; } }
}
