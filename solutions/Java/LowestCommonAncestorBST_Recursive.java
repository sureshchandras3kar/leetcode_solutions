public class LowestCommonAncestorBST_Recursive {
    static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
        if (root.val > p.val && root.val > q.val) {
            // Both p and q are in left subtree
            return lowestCommonAncestor(root.left, p, q);
        } else if (root.val < p.val && root.val < q.val) {
            // Both p and q are in right subtree
            return lowestCommonAncestor(root.right, p, q);
        } else {
            // p and q are on different sides or one of them is root
            return root;
        }
    }

    public static void main(String[] args) {
        TreeNode root = new TreeNode(6);
        root.left = new TreeNode(2);
        root.right = new TreeNode(8);
        root.left.left = new TreeNode(0);
        root.left.right = new TreeNode(4);
        root.left.right.left = new TreeNode(3);
        root.left.right.right = new TreeNode(5);
        root.right.left = new TreeNode(7);
        root.right.right = new TreeNode(9);

        LowestCommonAncestorBST_Recursive sol = new LowestCommonAncestorBST_Recursive();
        TreeNode p = root.left;
        TreeNode q = root.left.right;
        System.out.println(sol.lowestCommonAncestor(root, p, q).val);  // 2
    }
}
