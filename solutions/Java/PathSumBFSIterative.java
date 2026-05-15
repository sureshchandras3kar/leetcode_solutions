import java.util.*;

class Solution {
    public boolean hasPathSum(TreeNode root, int targetSum) {
        if (root == null) return false;
        
        Queue<TreeNode> nodeQueue = new LinkedList<>();
        Queue<Integer> sumQueue = new LinkedList<>();
        nodeQueue.offer(root);
        sumQueue.offer(root.val);
        
        while (!nodeQueue.isEmpty()) {
            TreeNode node = nodeQueue.poll();
            int sum = sumQueue.poll();
            
            if (node.left == null && node.right == null && sum == targetSum) {
                return true;
            }
            
            if (node.left != null) {
                nodeQueue.offer(node.left);
                sumQueue.offer(sum + node.left.val);
            }
            
            if (node.right != null) {
                nodeQueue.offer(node.right);
                sumQueue.offer(sum + node.right.val);
            }
        }
        
        return false;
    }
}

System.out.println("PathSum Solution");
