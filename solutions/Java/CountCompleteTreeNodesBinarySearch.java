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
    private boolean exists(long pos, int height, TreeNode root) {
        /**Check if node at position pos exists.*/
        long left = 0, right = (1L << (height - 1)) - 1;

        for (int i = 0; i < height - 1; i++) {
            long mid = (left + right + 1) / 2;
            if (pos >= mid) {
                root = root.right;
                left = mid;
            } else {
                root = root.left;
                right = mid - 1;
            }
        }

        return root != null;
    }

    /**
     * Count nodes in complete binary tree using binary search on node positions.
     * Uses existence check for node at each possible position.
     * Time: O(log² n), Space: O(log n) for recursion
     */
    public int countNodes(TreeNode root) {
        if (root == null) return 0;

        // Find height of tree
        int height = 0;
        TreeNode node = root;
        while (node != null) {
            height++;
            node = node.left;
        }

        // Binary search on number of nodes
        // For a complete tree of height h, nodes range from 2^(h-1) to 2^h - 1
        long low = 1L << (height - 1);   // 2^(h-1)
        long high = (1L << height) - 1;  // 2^h - 1

        while (low <= high) {
            long mid = (low + high + 1) / 2;
            if (exists(mid, height, root)) {
                low = mid;
            } else {
                high = mid - 1;
            }
        }

        return (int)low;
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
