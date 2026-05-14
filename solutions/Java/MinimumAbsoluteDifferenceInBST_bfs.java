import java.util.Stack;

public class MinimumAbsoluteDifferenceInBST_BFS {
    static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public int getMinimumDifference(TreeNode root) {
        Stack<TreeNode> stack = new Stack<>();
        TreeNode current = root;
        TreeNode prev = null;
        int minDiff = Integer.MAX_VALUE;

        while (!stack.isEmpty() || current != null) {
            // Go to leftmost node
            while (current != null) {
                stack.push(current);
                current = current.left;
            }

            // Current is null, pop from stack
            current = stack.pop();

            // Process current node
            if (prev != null) {
                minDiff = Math.min(minDiff, current.val - prev.val);
            }
            prev = current;

            // Visit right subtree
            current = current.right;
        }

        return minDiff;
    }

    public static void main(String[] args) {
        TreeNode root = new TreeNode(4);
        root.left = new TreeNode(2);
        root.right = new TreeNode(6);
        root.left.left = new TreeNode(1);
        root.left.right = new TreeNode(3);

        MinimumAbsoluteDifferenceInBST_BFS sol = new MinimumAbsoluteDifferenceInBST_BFS();
        System.out.println(sol.getMinimumDifference(root));  // 1
    }
}
