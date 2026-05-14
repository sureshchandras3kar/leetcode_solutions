import java.util.*;

public class Solution {
    private void dfs(TreeNode node, int level, List<Long> sums, List<Integer> counts) {
        if (node == null) return;

        if (level == sums.size()) {
            sums.add(0L);
            counts.add(0);
        }

        sums.set(level, sums.get(level) + node.val);
        counts.set(level, counts.get(level) + 1);

        dfs(node.left, level + 1, sums, counts);
        dfs(node.right, level + 1, sums, counts);
    }

    public List<Double> averageOfLevels(TreeNode root) {
        List<Long> sums = new ArrayList<>();
        List<Integer> counts = new ArrayList<>();
        dfs(root, 0, sums, counts);

        List<Double> result = new ArrayList<>();
        for (int i = 0; i < sums.size(); i++) {
            result.add((double) sums.get(i) / counts.get(i));
        }
        return result;
    }

    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int x) { val = x; }
    }

    public static void main(String[] args) {
        Solution sol = new Solution();
        TreeNode root = new TreeNode(3);
        root.left = new TreeNode(9);
        root.right = new TreeNode(20);
        root.right.left = new TreeNode(15);
        root.right.right = new TreeNode(7);

        List<Double> result = sol.averageOfLevels(root);
        System.out.println("Averages: " + result);
    }
}
