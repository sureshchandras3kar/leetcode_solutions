import java.util.Queue;
import java.util.LinkedList;

class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;

    TreeNode() {}
    TreeNode(int val) {
        this.val = val;
    }
}

class Pair {
    TreeNode node;
    int sum;

    Pair(TreeNode node, int sum) {
        this.node = node;
        this.sum = sum;
    }
}

class Solution {
    /**
     * Check if tree has root-to-leaf path summing to targetSum using iterative BFS.
     * Queue stores (node, current_sum) pairs.
     * Time: O(n), Space: O(w) where w is max width
     */
    public boolean hasPathSum(TreeNode root, int targetSum) {
        if (root == null) return false;

        Queue<Pair> queue = new LinkedList<>();
        queue.add(new Pair(root, root.val));

        while (!queue.isEmpty()) {
            Pair pair = queue.poll();
            TreeNode node = pair.node;
            int currentSum = pair.sum;

            // Check leaf node
            if (node.left == null && node.right == null && currentSum == targetSum) {
                return true;
            }

            if (node.left != null) {
                queue.add(new Pair(node.left, currentSum + node.left.val));
            }
            if (node.right != null) {
                queue.add(new Pair(node.right, currentSum + node.right.val));
            }
        }

        return false;
    }
}

class Main {
    public static void main(String[] args) {
        // Example 1: [5,4,8,11,null,13,4,7,2,null,1], targetSum = 22
        TreeNode root = new TreeNode(5);
        root.left = new TreeNode(4);
        root.left.left = new TreeNode(11);
        root.left.left.left = new TreeNode(7);
        root.left.left.right = new TreeNode(2);
        root.right = new TreeNode(8);
        root.right.left = new TreeNode(13);
        root.right.right = new TreeNode(4);
        root.right.right.right = new TreeNode(1);

        Solution sol = new Solution();
        System.out.println("Example 1 (target 22): " + sol.hasPathSum(root, 22));  // true
        System.out.println("Example 1 (target 20): " + sol.hasPathSum(root, 20));  // false
    }
}
