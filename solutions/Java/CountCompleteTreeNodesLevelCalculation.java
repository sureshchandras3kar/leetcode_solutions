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
    /**
     * Count nodes in complete binary tree using level calculation.
     * For complete tree, if left height == right height, left is perfect.
     * Time: O(log² n), Space: O(log n) for recursion
     */
    public int countNodes(TreeNode root) {
        if (root == null) return 0;

        // Calculate left and right heights
        int leftHeight = 0;
        TreeNode leftNode = root.left;
        while (leftNode != null) {
            leftHeight++;
            leftNode = leftNode.left;
        }

        int rightHeight = 0;
        TreeNode rightNode = root.right;
        while (rightNode != null) {
            rightHeight++;
            rightNode = rightNode.right;
        }

        if (leftHeight == rightHeight) {
            // Left subtree is perfect: 2^(h+1) - 1 nodes + root + recursively count right
            return (1 << (leftHeight + 1)) - 1 + countNodes(root.right);
        } else {
            // Right subtree is perfect: 2^h - 1 nodes + root + recursively count left
            return (1 << (rightHeight + 1)) - 1 + countNodes(root.left);
        }
    }
}

class Main {
    public static void main(String[] args) {
        // Example 1: Complete tree with 5 nodes
        TreeNode root = new TreeNode(1);
        root.left = new TreeNode(2);
        root.right = new TreeNode(3);
        root.left.left = new TreeNode(4);
        root.left.right = new TreeNode(5);

        Solution sol = new Solution();
        System.out.println("Example 1 node count: " + sol.countNodes(root));  // 5

        // Example 2: Single node
        TreeNode single = new TreeNode(1);
        System.out.println("Example 2 node count: " + sol.countNodes(single));  // 1

        // Example 3: Empty tree
        System.out.println("Example 3 node count: " + sol.countNodes(null));  // 0
    }
}
