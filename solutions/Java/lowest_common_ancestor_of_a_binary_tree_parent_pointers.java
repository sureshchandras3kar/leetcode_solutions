import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class Solution {
    private Map<TreeNode, TreeNode> parent = new HashMap<>();

    private void dfs(TreeNode node) {
        if (node == null) {
            return;
        }
        if (node.left != null) {
            parent.put(node.left, node);
            dfs(node.left);
        }
        if (node.right != null) {
            parent.put(node.right, node);
            dfs(node.right);
        }
    }

    /**
     * Find the lowest common ancestor using parent pointers and hash set.
     *
     * Time Complexity: O(n) where n is the number of nodes
     * Space Complexity: O(h) where h is the height
     */
    public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
        parent.clear();
        dfs(root);

        // Collect all ancestors of p
        Set<TreeNode> ancestors = new HashSet<>();
        TreeNode current = p;
        while (current != null) {
            ancestors.add(current);
            current = parent.get(current);
        }

        // Walk from q up to find first common ancestor
        current = q;
        while (!ancestors.contains(current)) {
            current = parent.get(current);
        }

        return current;
    }

    // TreeNode definition
    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;
        TreeNode(int x) {
            val = x;
        }
    }

    // Test cases
    public static void main(String[] args) {
        Solution solution = new Solution();

        // Example 1: [3,5,1,6,2,0,8,null,null,7,4]
        //       3
        //      / \
        //     5   1
        //    / \ / \
        //   6  2 0  8
        //     / \
        //    7   4
        TreeNode root1 = new TreeNode(3);
        root1.left = new TreeNode(5);
        root1.right = new TreeNode(1);
        root1.left.left = new TreeNode(6);
        root1.left.right = new TreeNode(2);
        root1.right.left = new TreeNode(0);
        root1.right.right = new TreeNode(8);
        root1.left.right.left = new TreeNode(7);
        root1.left.right.right = new TreeNode(4);

        TreeNode p1 = root1.left;  // Node 5
        TreeNode q1 = root1.right;  // Node 1
        TreeNode result1 = solution.lowestCommonAncestor(root1, p1, q1);
        System.out.println("LCA of " + p1.val + " and " + q1.val + ": " + result1.val);  // Expected: 3

        // Example 2: Same tree, p=5, q=4
        TreeNode p2 = root1.left;  // Node 5
        TreeNode q2 = root1.left.right.right;  // Node 4
        TreeNode result2 = solution.lowestCommonAncestor(root1, p2, q2);
        System.out.println("LCA of " + p2.val + " and " + q2.val + ": " + result2.val);  // Expected: 5

        // Example 3: [1,2]
        //     1
        //      \
        //       2
        TreeNode root3 = new TreeNode(1);
        root3.right = new TreeNode(2);

        TreeNode p3 = root3;  // Node 1
        TreeNode q3 = root3.right;  // Node 2
        TreeNode result3 = solution.lowestCommonAncestor(root3, p3, q3);
        System.out.println("LCA of " + p3.val + " and " + q3.val + ": " + result3.val);  // Expected: 1
    }
}
