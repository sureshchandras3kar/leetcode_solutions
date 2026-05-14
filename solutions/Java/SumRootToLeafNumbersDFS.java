class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;

    TreeNode() {}
    TreeNode(int val) {
        this.val = val;
    }
}

class Solution {
    private int dfs(TreeNode node, int currentSum) {
        if (node == null) return 0;

        // Build number: multiply by 10 and add current digit
        currentSum = currentSum * 10 + node.val;

        // Leaf node: return the complete number
        if (node.left == null && node.right == null) {
            return currentSum;
        }

        // Recursively process children and sum
        return dfs(node.left, currentSum) + dfs(node.right, currentSum);
    }

    /**
     * Sum all root-to-leaf numbers using DFS.
     * Build number by appending digits as we traverse down.
     * Time: O(n), Space: O(h) for recursion stack
     */
    public int sumNumbers(TreeNode root) {
        return dfs(root, 0);
    }
}

class Main {
    public static void main(String[] args) {
        // Example 1: [1,2,3]
        TreeNode root = new TreeNode(1);
        root.left = new TreeNode(2);
        root.right = new TreeNode(3);

        Solution sol = new Solution();
        System.out.println("Example 1 sum: " + sol.sumNumbers(root));  // 25 (12 + 13)

        // Example 2: [4,9,0,5,1]
        TreeNode root2 = new TreeNode(4);
        root2.left = new TreeNode(9);
        root2.right = new TreeNode(0);
        root2.left.left = new TreeNode(5);
        root2.left.right = new TreeNode(1);

        System.out.println("Example 2 sum: " + sol.sumNumbers(root2));  // 1026 (495 + 491 + 40)
    }
}
