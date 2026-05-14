public class Solution {
    long[] prev = {Long.MIN_VALUE};
    boolean dfs(TreeNode n) {
        if (n == null) return true;
        if (!dfs(n.left)) return false;
        if (n.val <= prev[0]) return false;
        prev[0] = n.val;
        return dfs(n.right);
    }
    public boolean isValidBST(TreeNode root) {
        return dfs(root);
    }
    public static class TreeNode { int val; TreeNode left, right; TreeNode(int x) { val = x; } }
}
