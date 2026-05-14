import java.util.*;

public class Solution {
    void dfs(TreeNode n, int level, List<List<Integer>> result) {
        if (n == null) return;
        if (level == result.size()) result.add(new ArrayList<>());
        result.get(level).add(n.val);
        dfs(n.left, level + 1, result);
        dfs(n.right, level + 1, result);
    }
    
    public List<List<Integer>> levelOrder(TreeNode root) {
        List<List<Integer>> result = new ArrayList<>();
        dfs(root, 0, result);
        return result;
    }
    
    public static class TreeNode {
        int val; TreeNode left, right;
        TreeNode(int x) { val = x; }
    }
}
