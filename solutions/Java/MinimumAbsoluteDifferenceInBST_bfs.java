import java.util.*;

class Solution {
    public int getMinimumDifference(TreeNode root) {
        int minDiff = Integer.MAX_VALUE;
        Integer prevVal = null;
        Queue<TreeNode> queue = new LinkedList<>();
        queue.offer(root);
        
        while (!queue.isEmpty()) {
            TreeNode node = queue.poll();
            if (node == null) continue;
            
            if (prevVal != null) {
                minDiff = Math.min(minDiff, Math.abs(node.val - prevVal));
            }
            prevVal = node.val;
            
            if (node.left != null) queue.offer(node.left);
            if (node.right != null) queue.offer(node.right);
        }
        
        return minDiff;
    }
}

System.out.println("MinDiff Solution");
