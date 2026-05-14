import java.util.*;

public class Solution {
    public List<List<Integer>> zigzagLevelOrder(TreeNode root) {
        List<List<Integer>> result = new ArrayList<>();
        if (root == null) return result;
        Queue<TreeNode> q = new LinkedList<>(); q.offer(root);
        int level = 0;
        while (!q.isEmpty()) {
            int sz = q.size(); Deque<Integer> dq = new LinkedList<>();
            for (int i = 0; i < sz; i++) {
                TreeNode n = q.poll();
                if (level % 2 == 0) dq.offer(n.val);
                else dq.offerFirst(n.val);
                if (n.left != null) q.offer(n.left);
                if (n.right != null) q.offer(n.right);
            }
            result.add(new ArrayList<>(dq));
            level++;
        }
        return result;
    }
    public static class TreeNode { int val; TreeNode left, right; TreeNode(int x) { val = x; } }
}
